use compress::compression::compressor::{calculate_byte_frequency, from_bytes_to_min_heap, build_graph, compress};
use compress::file::file::read_file;

fn main() {

    let bytes = read_file("./src/test_file.txt");
    match bytes {
        Ok(bytes) => {
            let hash_map = calculate_byte_frequency(bytes.clone());
            let mut min_heap = from_bytes_to_min_heap(hash_map.clone());
            let graph = build_graph(&mut min_heap);
            println!("{:?}", graph);
            match graph {
                Ok(graph) => {
                    println!("{:?}", compress(bytes.clone(), hash_map.clone() ,graph));
                },
                Err(error) => {
                    println!("Error: {:?}", error);
                }
            }
        }, 
        Err(error) => {
            println!("Error: {}", error);
        }
    }

}
