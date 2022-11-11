use compress::compression::compressor::{calculate_byte_frequency, from_bytes_to_min_heap, build_graph};
use compress::file::file::read_file;

fn main() {

    let bytes = read_file("./src/test_file.txt");
    match bytes {
        Ok(bytes) => {
            let hash_map = calculate_byte_frequency(bytes);
            let mut min_heap = from_bytes_to_min_heap(hash_map);
            let graph = build_graph(&mut min_heap);
            println!("{:?}", graph);
        }, 
        Err(error) => {
            println!("Error: {}", error);
        }
    }

}
