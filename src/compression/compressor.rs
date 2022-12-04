use std::collections::{BinaryHeap, HashMap};
use crate::{graph::{node::Node, graph::Graph}, utils::error::CompressionError};
use itertools::Itertools;
use std::fs::File;
use std::os::unix::fs::FileExt;

pub fn calculate_byte_frequency(bytes: Vec<u8>) -> HashMap<u8,usize> {
    bytes.into_iter().counts()
}

pub fn from_bytes_to_min_heap(frequencies: HashMap<u8, usize>) -> BinaryHeap<Node>{
    let mut min_heap = BinaryHeap::<Node>::new();
    for item in frequencies {
        let node = Node::new(Some(item.0), item.1, None, None);
        min_heap.push(node);
    }
    min_heap
}

pub fn build_graph(min_heap: &mut BinaryHeap<Node>) -> Result<Graph, CompressionError>{

    while min_heap.len() > 1 {
        let mut empty_node = Node::new_empty_node();

        let min_node_left = min_heap.pop().unwrap();
        let min_node_right = min_heap.pop().unwrap();

        empty_node.add_child(min_node_left)?;
        empty_node.add_child(min_node_right)?;

        min_heap.push(empty_node);
    }

    let root = min_heap.pop().unwrap();
    let graph = Graph::new(root);
    
    Ok(graph)
}

pub fn compress(bytes: Vec<u8>,hash_map: HashMap<u8, usize> , graph: Graph) {
    let mut compressed_bytes = HashMap::new();
    for byte in hash_map.keys() {
        let path = graph.search(*byte);
        match path {
            Some(path) => {
                compressed_bytes.insert(*byte, path);
            },
            None => println!("No path found for byte: {}", byte)
        }
    }
    println!("{:?}", compressed_bytes);

    let mut i = 0;
    let file = File::create("compressed.txt").unwrap();
    let len = bytes.len();
    while i < len {
        let mut bytes_to_write = Vec::<u8>::new();
        let mut j = 0;
        //Escribimos en un archivo nuevo de a 100
        while j < 100 && i + j < len {
            let byte = bytes[i + j];
            let path = compressed_bytes.get(&byte);
            match path {
                Some(path) => {
                    path.chars().for_each(|c| bytes_to_write.push(c as u8));
                },
                None => println!("No path found for byte: {}", byte)
            }
            j += 1;
        }
        file.write_at(&bytes_to_write,i.try_into().unwrap());
        i += j;
        println!("Compressed {:?}% of file", i*100/len);
    }


}