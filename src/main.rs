extern crate petgraph;

use std::collections::HashMap;
use petgraph::graphmap::{GraphMap,NodeTrait};

fn main() {
    println!("Hello, world!");
}



struct BronKerbosch<N: NodeTrait,E> {
    graph: GraphWrapper<N,E>
}

impl<N: NodeTrait,E> BronKerbosch<N,E> {
    fn new(graphmap: GraphMap<N,E>) -> BronKerbosch<N,E> {
        BronKerbosch {
            graph: GraphWrapper::new(graphmap)
        }
    }
}

struct GraphWrapper<N: NodeTrait,E> {
    graph: GraphMap<N,E>,
    node_map: HashMap<usize, N>
}

impl<N: NodeTrait,E> GraphWrapper<N,E> {
    fn new(graphmap: GraphMap<N,E>) -> GraphWrapper<N,E> {
        let mut node_map = HashMap::new();

        let mut node_list = graphmap.nodes().collect::<Vec<N>>();
        node_list.sort();

        for (i, val) in node_list.iter().enumerate() {
            node_map.insert(i, val.clone());
        }

        GraphWrapper {
            graph: graphmap,
            node_map: node_map
        }
    }

    fn connected(&self, node_index_a: usize, node_index_b: usize) -> bool {
        let node_a = self.node_map.get(&node_index_a).unwrap();
        let node_b = self.node_map.get(&node_index_b).unwrap();

        self.graph.contains_edge(node_a.clone(), node_b.clone())
    }
}