use std::{collections::{BinaryHeap, HashMap}, thread::sleep};
use crate::{graph::{node::Node, graph::Graph}, utils::error::CompressionError, file::{file::read_file}};
use itertools::Itertools;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::time::Duration;

const PERCENTAGE_COMPRESS: f32 = 0.1;
const BYTE_LENGTH: usize = 8;

#[derive(Debug)]
pub struct Compressor {
    pub filename: String,
    pub data: Vec<u8>,
    pub original_file_length: usize,
    pub compressed_file: File,
    pub compressed_file_length: usize,
    pub graph: Option<Graph>,
    pub logger: mpsc::Sender<usize>
}

impl Compressor {

    pub fn new(directory:String, filename: String, tx: mpsc::Sender<usize>, compress: bool) -> Result<Self, CompressionError> {

        let filepath = format!("{}{}", directory, filename);
        let bytes = read_file(&filepath)?;

        let path_vec: Vec<&str> = filename.split('.').collect();

        let filename = path_vec[0].to_string();

        let file_length = bytes.len();

        if !compress {
            let compressor = Compressor {
                filename: filename.clone(),
                data: Vec::new(),
                original_file_length: file_length,
                compressed_file: File::create(format!("./decompressed_files/{}_decompressed.txt",filename))?,
                compressed_file_length: 0,
                graph: None, 
                logger: tx
            };
            return Ok(compressor);
        } 

        let compressed_filename = format!("./compressed_files/{}_compressed",filename);
        
        let compressed_file = File::create(compressed_filename)?;

        let compressor = Compressor {
            filename,
            data: bytes,
            original_file_length: file_length,
            compressed_file,
            compressed_file_length: 0,
            graph: None,
            logger: tx
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

    pub fn start_decompressor(&mut self) -> Result<(), CompressionError> {

        self.decompress()?;

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


        Ok(compression_dictionary)
    }

    pub fn compress(&mut self, frecuency_dictionary: HashMap<u8, usize>) -> Result<(),CompressionError>{
    
        let compression_dictionary = self.make_byte_dictionary(frecuency_dictionary.clone())?;
        //println!("{:?}", frecuency_dictionary);
        // println!("{:?}", compression_dictionary);

        let mut compressed_string = String::new();
        let mut compressed_bytes = Vec::<u8>::new();

        for byte in self.data.iter() {
            let path = compression_dictionary.get(&byte);
            compressed_string = format!("{}{}", compressed_string, path.unwrap());
        }

        let mut size_tmp_compressed = 0.0;
        let mut percentage_compressed;
        let mut byte_compressed;
        
        while compressed_string.len() > BYTE_LENGTH {

            size_tmp_compressed += 1.0;

            let first_eight = &compressed_string[0..8];
            let tail = &compressed_string[8..];
            byte_compressed = u8::from_str_radix(&first_eight, 2)?;
            compressed_string = format!("{}", tail);
            compressed_bytes.push(byte_compressed);

            percentage_compressed = size_tmp_compressed/(self.original_file_length as f32);
                
            if percentage_compressed >=  PERCENTAGE_COMPRESS {
                sleep(Duration::from_millis(1000));
                self.logger.send(1)?;
                size_tmp_compressed = 0.0;
            }
        }

        let last_compressed_string = format!("{:0>8}", compressed_string);
        let last_string_length = compressed_string.len() as u8;

        byte_compressed = u8::from_str_radix(&last_compressed_string, 2)?;
        compressed_bytes.push(byte_compressed);
        self.logger.send(1)?;
        
        serde_json::to_writer(&self.compressed_file, &frecuency_dictionary).unwrap();
        self.compressed_file.write(&compressed_bytes)?;
        self.compressed_file.write(&[last_string_length])?;

        Ok(())
    
    }


    pub fn decompress(&mut self) -> Result<(), CompressionError> {
        
        let file_path = format!("./compressed_files/{}", self.filename);
        let file_content = read_file(&file_path)?;
        let mut frequency_bytes = Vec::<u8>::new();
        let mut i = 0;

        for byte in file_content.iter() {
            if *byte == " ".as_bytes()[0] {
                continue;
            }
            frequency_bytes.push(*byte);
            i += 1;
            if *byte == "}".as_bytes()[0]{
                break;
            }
        }

        let frecuency_dictionary_str = std::str::from_utf8(&frequency_bytes).unwrap();
        
        let frecuency_dictionary:HashMap<u8, usize> = serde_json::from_str(&frecuency_dictionary_str).unwrap();

        let mut min_heap = self.from_bytes_to_min_heap(frecuency_dictionary.clone());
        let mut graph = self.build_graph(&mut min_heap)?;

        //println!("{:?}", self.graph);
        let mut bit_data = String::new();

        let mut size_tmp_decompressed = 0.0;
        let mut percentage_decompressed;

        while i < file_content.len() - 2{

            size_tmp_decompressed += 1.0;

            let byte = file_content[i];

            let bit_string = format!("{:0>8}", format!("{:b}", byte));
            bit_data = format!("{}{}", bit_data, bit_string);
            i += 1;

            percentage_decompressed = size_tmp_decompressed/(self.original_file_length as f32);
                
            if percentage_decompressed >=  PERCENTAGE_COMPRESS {
                sleep(Duration::from_millis(1000));
                self.logger.send(1)?;
                size_tmp_decompressed = 0.0;
            }
        }

        //Esto es solo para el ultimo para que no tenga 0's al principio
        let last_byte = file_content[i];
        let bit_string = format!("{:b}", last_byte);

        let length_last_byte = file_content[i + 1] as usize;
        let num_of_zeros = length_last_byte - bit_string.len();
        let zeros = std::iter::repeat("0").take(num_of_zeros).collect::<String>();
        let last_bit_string = format!("{}{}", zeros, bit_string);
        bit_data = format!("{}{}", bit_data, last_bit_string);

        for c in bit_data.chars() {
            if c == '0' {
                graph.go_left()?;
            } else {
                graph.go_right()?;
            }
            if graph.is_leaf() {
                self.compressed_file.write(&[graph.get_byte().unwrap()])?;
                graph.reset();
            }
        }
        Ok(()) 
    }

    pub fn compress_old(&mut self, frecuency_dictionary: HashMap<u8, usize>) -> Result<(),CompressionError>{
    
        let compression_dictionary = self.make_byte_dictionary(frecuency_dictionary)?;

        let mut compressed_string = String::new();
        let mut compressed_bytes = Vec::<u8>::new();

        let mut size_tmp_compressed = 0.0;
        let mut percentage_compressed;

        for byte in self.data.iter() {

            size_tmp_compressed += 1.0;

            let path = compression_dictionary.get(&byte);
            
            if compressed_string.len() >= BYTE_LENGTH {

                let byte_compressed;
                
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

                percentage_compressed = size_tmp_compressed/(self.original_file_length as f32);
                
                if percentage_compressed >=  PERCENTAGE_COMPRESS {
                    sleep(Duration::from_millis(1000));
                    self.logger.send(1)?;
                    size_tmp_compressed = 0.0;
                }

                

            }
            compressed_string = format!("{}{}", compressed_string, path.unwrap());
        }
        
        self.logger.send(1)?;
        self.compressed_file.write(&compressed_bytes)?;
    
        Ok(())
    
    }
}





#[cfg(test)]
mod tests {
   
    use std::sync::mpsc::{Sender, Receiver, self};
    use crate::{file::file::read_file, graph::{node::Node, graph::Graph}, compression::compressor::Compressor};

 

    #[test]
    fn calculate_byte_frequency_works_ok_test() {
    let (tx, rx): (Sender<usize>, Receiver<usize>) = mpsc::channel();
    let directory = "./source_files/".to_string();
    let compressor = Compressor::new(directory, "test_compressions.txt".to_string(), tx, true);
    //return vec<u8>
    
    let hash_map = compressor.unwrap().calculate_byte_frequency();
    println!("{:?}",hash_map);
    assert_eq!(hash_map.contains_key(&116), true);
    assert_eq!(hash_map.contains_key(&101), true);
    assert_eq!(hash_map.contains_key(&115), true);


    for (key, value) in hash_map.iter() {
        assert_eq!(value,&1)
    
    }

    }
  
    #[test]
    fn build_graph_works_ok_test() {

  
    let (tx, rx): (Sender<usize>, Receiver<usize>) = mpsc::channel();
    let directory = "./source_files/".to_string();
    let compressor = Compressor::new(directory, "test_compressions.txt".to_string(), tx, true).unwrap();

    //return vec<u8>
    let bytes = read_file("./src/test_compressions.txt");
    let hash_map = compressor.calculate_byte_frequency();
    let mut min_heap = compressor.from_bytes_to_min_heap(hash_map);
    let graph = compressor.build_graph(&mut min_heap);
    println!("{:?}",graph);


    let root = Node::new(
        None,
        3,
        Some(Box::new(
            Node::new(
            Some(115),
            1,
            None,
            None,
        ))),
        Some(Box::new(
            Node::new(
            None,
            2,
            Some(Box::new(
                Node::new(
                Some(116),
                1,
                None,
                None,
            ))),
            Some(Box::new(Node::new(
                Some(101),
                1,
                None,
                None
            ))
        ))),
    ));
    let graphMock = Graph::new(root);
    assert_eq!(graph.unwrap(),graphMock)

    } 
}