
extern crate petgraph;

mod bronkerbosch;
mod graph_wrapper;
mod tomita;

use bronkerbosch::BronKerbosch;
use tomita::Tomita;
use graph_wrapper::GraphWrapper;

use petgraph::graphmap::{GraphMap,NodeTrait};

use std::io;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

pub fn loadFromFile(filename: &str) -> GraphMap<i32, ()> {
    let mut graph = GraphMap::new();
    let f = match File::open(filename) {
        Ok(v) => v,
        Err(e) => return graph
    };
    let mut reader = BufReader::new(f);

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => break 
        };

        if line.starts_with("#") { continue }
        let v : Vec<&str> = line.split_whitespace().collect();
        let n0 = v[0].parse::<i32>().unwrap();
        let n1 = v[1].parse::<i32>().unwrap();
        graph.add_node(n0); graph.add_node(n1);
        if(n0 != n1) { graph.add_edge(n0, n1, ()); }
    }
    return graph;
}

fn print_help() {
    println!("Clique Enumerator. Syntax:\n programname graph_file method \n 
             mathed can be: \"BK\" or \"TOMITA\".");
}

fn main() {
    let args : Vec<_> = env::args().collect();
    let argv: Vec<&str> = args.iter().map(|x| x.as_ref() ).collect();
    if args.len() > 2 {
        let graph4 = loadFromFile(argv[1]);
        match args[2].to_uppercase().as_ref() {
            "BK" => {
                println!("B-K-Algorithm: ");
                let mut bk4 = BronKerbosch::new(graph4);
                bk4.compute();
                for c in bk4.cliques().iter() {
                    println!("{:?}", c);
                }
            },
            "TOMITA" => {
                println!("Tomita-Algorithm: ");
                let graph4 = loadFromFile(argv[1]);
                let mut tomita = Tomita::new(graph4);
                tomita.compute();
                for c in tomita.cliques().iter() {
                    println!("{:?}", c);
                }
            }
            _ => { print_help() }
        };
    } else { print_help() }
}
