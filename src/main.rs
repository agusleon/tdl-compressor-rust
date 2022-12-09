use compress::compression::compressor::Compressor;



fn main() {

    let directory = "./src/".to_string();

    match Compressor::new(directory, "car.png".to_string()) {
        Ok(mut compressor) => {
            match compressor.start_compressor() {
                Ok(_) => {println!("Finished compressing!")},
                Err(error) => {println!("Error: {}", error)}
            }
        },
        Err(error) => {
            println!("{}", error);
        }
    }

}
