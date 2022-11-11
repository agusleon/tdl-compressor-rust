use super::node::Node;

#[derive(Debug)]
pub struct Graph {
    pub root: Node
}

impl Graph {

    pub fn new(root: Node) -> Self{
        Graph {
            root
        }
    }
    
}

// Implementar el trait display para ver lindo el grafo

// impl fmt::Display for Graph {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
//     }
// }