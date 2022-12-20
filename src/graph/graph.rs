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



// Implementar el trait display para ver lindo el grafo

// impl fmt::Display for Graph {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
//     }
// }