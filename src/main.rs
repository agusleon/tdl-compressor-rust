use compress::compression::compressor::Compressor;
use compress::file;
use compress::file::file::read_arguments;
use compress::utils::error::CompressionError;
use compress::utils::logger::create_multiprogress_bar;
use std::sync::mpsc::{Sender, Receiver};
use std::{env, thread};
use std::process::exit;
use std::thread::JoinHandle;
use indicatif::{ProgressBar, MultiProgress, ProgressStyle};
use std::sync::{mpsc, Arc, Mutex};

const COMPRESS: &str = "COMPRESS";
const DECOMPRESS: &str = "DECOMPRESS"; 


fn main() {

    // Leemos argumentos por consola

    let args: Vec<String> = env::args().collect();

    let (filenames, processor) = match read_arguments(args) {
        Ok((vec_files, processor_type)) => (vec_files, processor_type),
        Err(error) => {
            println!("{}", error);
            exit(-1);
        }
    };

    match process(filenames, processor) {
        Ok(()) => {},
        Err(err) => {println!("Error: {}", err);}
    }

}

fn process(filenames: Vec<String>, processor_type: String) -> Result<(), CompressionError> {
    // Creamos multiprogress bar

    let (multiprogress_bar, style) = create_multiprogress_bar();
    // Por cada file creamos un thread e iniciamos una progress bar

    let mut processor_threads = Vec::<JoinHandle<()>>::new();
    let mut logger_threads = Vec::<JoinHandle<()>>::new();

    // La variable tiene que estar envuelta en un Arc (porque va a ser referenciada y modificada por varios owners) 
    // y ademas por un Mutex porque va a ser utilizado a lo largo de varios threads.

    let multiprogress_bar_mutex = Arc::new(Mutex::new(multiprogress_bar));

    if processor_type.to_uppercase() == COMPRESS {
        for file in filenames {

            // Se crea un channel por cada file, el sender va a ir al processor y el receiver al logger
            let (tx, rx): (Sender<usize>, Receiver<usize>) = mpsc::channel();
    
            // start logger
            logger(&multiprogress_bar_mutex.clone(), style.clone(), &mut logger_threads, rx, file.clone())?;
            
            // start processing
            compress(file, &mut processor_threads, tx)?;
        }
    } else if processor_type.to_uppercase() == DECOMPRESS {
        for file in filenames {

            // Se crea un channel por cada file, el sender va a ir al processor y el receiver al logger
            let (tx, rx): (Sender<usize>, Receiver<usize>) = mpsc::channel();
    
            // start logger
            logger(&multiprogress_bar_mutex.clone(), style.clone(), &mut logger_threads, rx, file.clone())?;
            
            // start processing
            decompress(file, &mut processor_threads, tx)?;
        }
    } else {
        println!("No processor type");
    }

    // Se espera a que terminen todos los threads
    for thread in processor_threads {
        thread.join().unwrap();
    }

    for thread in logger_threads {
        thread.join().unwrap();
    }


    {
        let multiprogress = multiprogress_bar_mutex.lock().map_err(|_| CompressionError::PoisonError)?;
        multiprogress.clear().unwrap();
    }

    Ok(())
}

fn compress(file: String, threads: &mut Vec<JoinHandle<()>>, tx: Sender<usize>) -> Result<(), CompressionError> {
    
    let directory = "./source_files/".to_string();

    // Se crean los threads uno por cada archivo

    let thread = thread::spawn(move || {

        match Compressor::new(directory, file, tx, true) {
            Ok(mut compressor) => {
                match compressor.start_compressor() {
                    Ok(_) => {},
                    Err(error) => {println!("Error: {}", error)}
                }
            },
            Err(error) => {
                println!("{}", error);
            }
        }

    });

    // Se agrega cada thread en una lista de threads para despues esperarlos a que terminen
    threads.push(thread);

    Ok(())

}

fn decompress(file: String, threads: &mut Vec<JoinHandle<()>>, tx: Sender<usize>) -> Result<(), CompressionError> {
    
    let directory = "./compressed_files/".to_string();

    // Se crean los threads uno por cada archivo

    let thread = thread::spawn(move || {

        match Compressor::new(directory, file, tx, false) {
            Ok(mut compressor) => {
                match compressor.start_decompressor() {
                    Ok(_) => {println!("Finished compressing!")},
                    Err(error) => {println!("Error: {}", error)}
                }
            },
            Err(error) => {
                println!("{}", error);
            }
        }

    });

    // Se agrega cada thread en una lista de threads para despues esperarlos a que terminen
    threads.push(thread);

    Ok(())

}

fn logger(multiprogress_bar_mutex: &Arc<Mutex<MultiProgress>>, style: ProgressStyle, threads: &mut Vec<JoinHandle<()>>, rx: mpsc::Receiver<usize>, filename: String) -> Result<(), CompressionError> {

    // Ponemos los corchetes porque realizamos un lock y al salir de scope el mismo dropea automaticamente
    let pb = {
        let multiprogress = multiprogress_bar_mutex.lock().map_err(|_| CompressionError::PoisonError)?;
        multiprogress.add(ProgressBar::new(10))
    };
    pb.set_style(style.clone());

    // Se crean los threads uno por cada archivo

    let thread = thread::spawn(move || {

        for _ in rx {
            pb.inc(1)
        }

    });


    
    // Se agrega cada thread en una lista de threads para despues esperarlos a que terminen
    threads.push(thread);

    Ok(())

}