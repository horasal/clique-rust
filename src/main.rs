#![feature(negate_unsigned)]

extern crate petgraph;

mod bronkerbosch;
mod graph_wrapper;

use bronkerbosch::BronKerbosch;
use graph_wrapper::GraphWrapper;

use petgraph::graphmap::{GraphMap,NodeTrait};

fn main() {
    println!("First graph's maximal cliques are:");
    let graph1 = graph1();
    let mut bk1 = BronKerbosch::new(graph1);
    bk1.compute();
    for c in bk1.cliques().iter() {
        println!("{:?}", c);
    }
    println!("------");

    println!("Second graph's maximal cliques are:");
    let graph2 = graph2();
    let mut bk2 = BronKerbosch::new(graph2);
    bk2.compute();
    for c in bk2.cliques().iter() {
        println!("{:?}", c);
    }
    println!("------");

    println!("Third graph's maximal cliques are:");
    let graph3 = graph3();
    let mut bk3 = BronKerbosch::new(graph3);
    bk3.compute();
    for c in bk3.cliques().iter() {
        println!("{:?}", c);
    }
    println!("------");

}

fn graph1() -> GraphMap<&'static str,()> {
    //
    //  A---B---C
    //
    let mut graph = GraphMap::new();
    graph.add_node("A");
    graph.add_node("B");
    graph.add_node("C");
    graph.add_edge("A", "B", ());
    graph.add_edge("B", "C", ());

    graph
}

fn graph2() -> GraphMap<&'static str,()> {
    //    -B-
    //   / | \
    //  A  |  D
    //   \ | /
    //    -C-
    let mut graph = GraphMap::new();
    graph.add_node("A");
    graph.add_node("B");
    graph.add_node("C");
    graph.add_node("D");
    graph.add_edge("A", "B", ());
    graph.add_edge("B", "D", ());
    graph.add_edge("A", "C", ());
    graph.add_edge("C", "D", ());
    graph.add_edge("B", "C", ());

    graph
}

fn graph3() -> GraphMap<&'static str,()> {
    //    -B---D
    //   / |\ /|
    //  A  | X |
    //   \ |/ \|
    //    -C---E
    let mut graph = GraphMap::new();
    graph.add_node("A");
    graph.add_node("B");
    graph.add_node("C");
    graph.add_node("D");
    graph.add_node("E");
    graph.add_edge("A", "B", ());
    graph.add_edge("A", "C", ());
    graph.add_edge("B", "C", ());
    graph.add_edge("B", "D", ());
    graph.add_edge("B", "E", ());
    graph.add_edge("C", "D", ());
    graph.add_edge("C", "E", ());
    graph.add_edge("D", "E", ());

    graph
}



