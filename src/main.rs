
extern crate petgraph;

mod bronkerbosch;
mod graph_wrapper;
mod tomita;
mod kplex;
mod kcore;

use bronkerbosch::BronKerbosch;
use tomita::Tomita;
use kplex::Kplex;
use kcore::KCore;

use petgraph::graphmap::{GraphMap};

use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

pub fn load_from_file(filename: &str) -> GraphMap<i32, ()> {
    let mut graph = GraphMap::new();
    let f = match File::open(filename) {
        Ok(v) => v,
        _ => return graph
    };
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            _ => break 
        };

        if line.starts_with("#") { continue }
        let v : Vec<&str> = line.split_whitespace().collect();
        let n0 = v[0].parse::<i32>().unwrap();
        let n1 = v[1].parse::<i32>().unwrap();
        graph.add_node(n0); graph.add_node(n1);
        if n0 != n1 { graph.add_edge(n0, n1, ()); }
    }
    return graph;
}

fn print_help() {
    println!("Clique Enumerator. Syntax:\n programname graph_file method \n 
             mathed can be: \"BK\" , \"TOMITA\", \"kplex\".");
}

fn main() {
    let args : Vec<_> = env::args().collect();
    let argv: Vec<&str> = args.iter().map(|x| x.as_ref() ).collect();
    if args.len() > 2 {
        let graph = load_from_file(argv[1]);
        match args[2].to_uppercase().as_ref() {
            "BK" => {
                println!("B-K-Algorithm: ");
                let mut bk = BronKerbosch::new(graph);
                bk.compute();
                for c in bk.cliques().iter() {
                    println!("{:?}", c);
                }
            },
            "TOMITA" => {
                println!("Tomita-Algorithm: ");
                let mut tomita = Tomita::new(graph);
                tomita.compute();
                for c in tomita.cliques().iter() {
                    println!("{:?}", c);
                }
            }
            "KPLEX" => {
                if args.len() > 3 {
                    let k = args[3].parse::<usize>().unwrap();
                    println!("KPlex: ");
                    let mut kplex = Kplex::new(graph, k);
                    kplex.compute();
                    for c in kplex.cliques().iter() {
                        println!("{:?}", c);
                    }
                }
            }
            "KCORE" => {
                if args.len() > 3 {
                    let k = args[3].parse::<usize>().unwrap();
                    println!("Kcore: ");
                    let mut kcore = KCore::new(graph, k);
                    kcore.compute();
                    for c in kcore.cliques().iter() {
                        println!("{:?}", c);
                    }
                }
            }
            _ => { print_help() }
        };
    } else { print_help() }
}
