use std::cmp::Ordering;
use crate::utils::error::CompressionError;

#[derive(Debug)]
pub struct Node {
    pub byte: Option<u8>,
    pub frequency: usize,
    pub node_left: Option<Box<Node>>,
    pub node_right: Option<Box<Node>>
}

impl Node {

    pub fn new(byte: Option<u8>, frequency: usize, node_left: Option<Box<Node>>, node_right: Option<Box<Node>>) -> Self{
        
        let node = Node {
            byte,
            frequency,
            node_left,
            node_right
        };
        // println!("A new node was created: {:?}", node);
        node
    }

    pub fn new_empty_node() -> Self {
        let node = Node {
            byte: None,
            frequency: 0,
            node_left: None,
            node_right: None
        };
        // println!("A new empty node was created: {:?}", node);
        node
    }

    pub fn add_child(&mut self, child: Node) -> Result<(), CompressionError>{

        let frequency = child.frequency;

        if self.node_right.is_some() {
            return Err(CompressionError::FullNode); 
        } else if self.node_left.is_none(){
            self.node_left = Some(Box::new(child));
        } else {
            self.node_right = Some(Box::new(child));
        }
        self.frequency += frequency;
        Ok(())
    }

    pub fn search(&self, byte: u8, mut path: String) -> Option<String> {
        
        if self.byte == Some(byte) {
            return Some(path);
        }
        else if self.node_left.is_some(){

            path.push('0');

            let new_path = self.node_left.as_ref().unwrap().search(byte, path.clone());
            
            match new_path{
                Some(new_path) => return Some(new_path),
                None => {
                    path.pop();
                    if self.node_right.is_some(){
                        path.push('1');
                        let new_path = self.node_right.as_ref().unwrap().search(byte, path.clone());
                        match new_path{
                            Some(new_path) => return Some(new_path),
                            None => {
                                path.pop();
                                return None;
                            }
                        }
                    }
                    else{
                        return None;
                    }
                }
            }
        }
        else{
            return None;
        }
    }

}

// We implement some traits so we can use the BinaryHeap object as a MinHeap directly with the Node object

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        self.frequency.cmp(&other.frequency).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.frequency == other.frequency
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            byte: self.byte,
            frequency: self.frequency,
            node_left: self.node_left.clone(),
            node_right: self.node_right.clone()
        }
    }
}