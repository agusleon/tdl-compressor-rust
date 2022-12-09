use std::collections::{BinaryHeap, HashMap};
use crate::{graph::{node::Node, graph::Graph}, utils::error::CompressionError, file::{file::read_file}};
use itertools::Itertools;
use std::fs::File;
use std::io::Write;

const BYTE_LENGTH: usize = 8;

#[derive(Debug)]
pub struct Compressor {
    pub filename: String,
    pub data: Vec<u8>,
    pub original_file_length: usize,
    pub compressed_file: File,
    pub compressed_file_length: usize,
    pub graph: Option<Graph>
}

impl Compressor {

    pub fn new(directory:String, filename: String) -> Result<Self, CompressionError> {

        let filepath = format!("{}{}", directory, filename);
        let bytes = read_file(&filepath)?;
        println!("{}", filepath);
        let path_vec: Vec<&str> = filename.split('.').collect();
        println!("{:?}", path_vec);
        let filename = path_vec[0].to_string();
        println!("{}", filename);
        let file_length = bytes.len();

        let compressed_filename = format!("./compressed_files/{}_compressed",filename);
        let compressed_file = File::create(compressed_filename)?;

        let compressor = Compressor {
            filename,
            data: bytes,
            original_file_length: file_length,
            compressed_file,
            compressed_file_length: 0,
            graph: None
        };

        Ok(compressor)
    }

    pub fn add_graph(&mut self, graph: Graph) {
        self.graph = Some(graph);
    }

    pub fn start_compressor(&mut self) -> Result<(), CompressionError> {

        let hash_map = self.calculate_byte_frequency();
        let mut min_heap = self.from_bytes_to_min_heap(hash_map.clone());
        let graph = self.build_graph(&mut min_heap)?;
        self.add_graph(graph);

        self.compress(hash_map.clone())?;

        Ok(())

    }

    pub fn calculate_byte_frequency(&self) -> HashMap<u8,usize> {
        self.data.clone().into_iter().counts()
    }
    
    pub fn from_bytes_to_min_heap(&self, frequencies: HashMap<u8, usize>) -> BinaryHeap<Node>{
        let mut min_heap = BinaryHeap::<Node>::new();
        for item in frequencies {
            let node = Node::new(Some(item.0), item.1, None, None);
            min_heap.push(node);
        }
        min_heap
    }
    
    pub fn build_graph(&self, min_heap: &mut BinaryHeap<Node>) -> Result<Graph, CompressionError>{
    
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
    
    pub fn make_byte_dictionary(&self, frecuency_dictionary: HashMap<u8, usize>) -> Result<HashMap<u8, String>, CompressionError>{
    
        let mut compression_dictionary = HashMap::new();

        match &self.graph {
            Some(graph) => {
                for byte in frecuency_dictionary.keys() {

                    let path = graph.search(*byte)?;
                    compression_dictionary.insert(*byte, path);

                }
            },
            None => {return Err(CompressionError::FullNode)}
        }
    
        println!("{:?}", compression_dictionary);

        Ok(compression_dictionary)
    }

    pub fn from_hashmap_to_string(&self, compression_dictionary: HashMap<u8, String>) {

        for (key,value) in compression_dictionary {
            
        }
    }
    
    pub fn compress(&mut self, frecuency_dictionary: HashMap<u8, usize>) -> Result<(),CompressionError>{
    
        let compression_dictionary = self.make_byte_dictionary(frecuency_dictionary)?;
        println!("{:?}", compression_dictionary);


    
        let mut i = 0;
        let mut compressed_string = String::new();
        let mut compressed_bytes = Vec::<u8>::new();

        println!("{:?}",self.data);

        for byte in self.data.iter() {

            let path = compression_dictionary.get(&byte);
            println!("{:?} : {:?}",byte, path.unwrap());

            
            if compressed_string.len() >= BYTE_LENGTH {

                let mut byte_compressed = 0;
                
                if compressed_string.len() > BYTE_LENGTH {
                    let first_eight = &compressed_string[0..8];
                    let tail = &compressed_string[8..];
                    byte_compressed = u8::from_str_radix(&first_eight, 2)?;
                    compressed_string = format!("{}", tail);
                } else {
                    byte_compressed = u8::from_str_radix(&compressed_string, 2)?;
                    compressed_string = String::new();
                }

                compressed_bytes.push(byte_compressed);
                
                println!("{}", compressed_string);

            }
            compressed_string = format!("{}{}", compressed_string, path.unwrap());
        }

        println!("{:?}",compressed_bytes);
        self.compressed_file.write(&compressed_bytes)?;
    
        Ok(())
    
    }
}



