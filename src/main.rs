use compress::compression::compressor::Compressor;



fn main() {

    let directory = "./src/".to_string();

    match Compressor::new(directory, "test_agus_compressed".to_string(), false) {
        Ok(mut compressor) => {
            /* match compressor.start_compressor() {
                Ok(_) => {println!("Finished compressing!")},
                Err(error) => {println!("Error: {}", error)}
            } */
            println!("Starting decompression...");
            match compressor.decompress("./compressed_files/test_agus_compressed".to_string()) {
                Ok(_) => {println!("Finished decompressing!")},
                Err(error) => {println!("Error: {}", error)}
            }
        },
        Err(error) => {
            println!("{}", error);
        }
    }

}
