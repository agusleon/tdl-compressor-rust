use std::collections::{BinaryHeap, HashMap};
use crate::{graph::{node::Node, graph::Graph}, utils::error::CompressionError, file::{file::read_file}};
use itertools::Itertools;
use std::fs::File;
use std::os::unix::fs::FileExt;

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
    
    pub fn compress(&self, frecuency_dictionary: HashMap<u8, usize>) -> Result<(),CompressionError>{
    
        let compression_dictionary = self.make_byte_dictionary(frecuency_dictionary)?;
        println!("{:?}", compression_dictionary);
        println!("BYTES: {:?}", self.data);
    
        let mut i = 0;
    
    
        while i < self.original_file_length {
    
            let mut compressed_bytes = Vec::<u8>::new();
            let mut compressed_string = String::new();
            let mut j = 0;
    
            //Escribimos en un archivo nuevo de a 100
            while j < 100 && i + j < self.original_file_length {
    
                let byte = self.data[i + j];
                let path = compression_dictionary.get(&byte);
    
                match path {
                    Some(path) => {
    
                        for char in path.chars() {
                            if compressed_string.len() == BYTE_LENGTH {
                                
                                let byte = u8::from_str_radix(&compressed_string, 2)?;
    
                                compressed_bytes.push(byte);
                                compressed_string = String::new();
                            }
                            
                            compressed_string = format!("{}{}", compressed_string, char);
    
                        }
                        
                    },
                    None => return Err(CompressionError::FullNode)
                }
    
                j += 1;
            }
    
            let left_bits = BYTE_LENGTH - compressed_string.len();
            let complete_zeros = std::iter::repeat("0").take(left_bits).collect::<String>();
            compressed_string = format!("{}{}", compressed_string, complete_zeros);
    
            let byte = u8::from_str_radix(&compressed_string, 2)?;
            compressed_bytes.push(byte);
    
            self.compressed_file.write_at(&compressed_bytes,i.try_into().unwrap())?;
            
            i += j;
    
            println!("Compressed {:?}% of file", i*100/self.original_file_length);
        }
    
        Ok(())
    
    }
}



