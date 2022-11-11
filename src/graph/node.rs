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