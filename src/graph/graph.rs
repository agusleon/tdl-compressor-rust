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

    pub fn search(&self, byte: u8) -> Option<String> {
        self.root.search(byte, String::new())
    }
    
}



// Implementar el trait display para ver lindo el grafo

// impl fmt::Display for Graph {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
//     }
// }