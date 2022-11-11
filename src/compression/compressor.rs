use std::collections::{BinaryHeap, HashMap};
use crate::{graph::{node::Node, graph::Graph}, utils::error::CompressionError};
use itertools::Itertools;

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

pub fn compress() {}