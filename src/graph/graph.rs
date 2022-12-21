use crate::utils::error::CompressionError;

use super::node::Node;

#[derive(Debug)]
pub struct Graph {
    pub root: Node,
    pub current_node: Node,
}

impl Graph {

    pub fn new(root: Node) -> Self{
        Graph {
            root: root.clone(),
            current_node: root
        }
    }

    pub fn search(&self, byte: u8) -> Result<String, CompressionError> {

        match self.root.search(byte, String::new()) {
            Some(path) => {return Ok(path)},
            None => {return Err(CompressionError::FullNode)}
        }
        
        
    }
    pub fn go_left(&mut self) -> Result<(), CompressionError> {
        match &self.current_node.node_left {
            Some(node) => {
                self.current_node = *node.clone();
                Ok(())
            },
            None => {return Err(CompressionError::FullNode)}
        }
    }
    pub fn go_right(&mut self) -> Result<(), CompressionError> {
        match &self.current_node.node_right {
            Some(node) => {
                self.current_node = *node.clone();
                Ok(())
            },
            None => {return Err(CompressionError::FullNode)}
        }
    }

    pub fn reset(&mut self) {
        self.current_node = self.root.clone();
    }
    pub fn is_leaf(&self) -> bool {
        self.current_node.node_left.is_none() && self.current_node.node_right.is_none()
    }

    pub fn get_byte(&self) -> Option<u8> {
        self.current_node.byte
    }

    
}

impl PartialEq for Graph{
    fn eq(&self, other: &Graph) -> bool {
        self.root == other.root
    }
}