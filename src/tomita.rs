use std::collections::HashSet;
use graph_wrapper::GraphWrapper;
use petgraph::graphmap::{GraphMap,NodeTrait};

/// Implementation according to "Algorithm 457: Finding All Cliques of an Undirected Graph"
/// by Bronand Kerbosch; http://doi.acm.org/10.1145/362342.362367
///
/// connected is a symmetrical bolean matrix, N the number of nodes in the graph,
/// values of the diagonal should be true.
pub struct Tomita<N: NodeTrait,E> {
    graph: GraphMap<N,E>,
    max_cliques: Vec<HashSet<N>>
}

impl<N: NodeTrait,E> Tomita<N,E> {
    pub fn new(graphmap: GraphMap<N,E>) -> Tomita<N,E> {
        Tomita {
            graph: graphmap,
            max_cliques: Vec::new()
        }
    }

    pub fn compute(&mut self) {
        let p = self.graph.nodes().collect::<HashSet<N>>();
        let r = HashSet::new();
        let x = HashSet::new();
        self.tomita(p, r, x);
    }

    pub fn cliques(&self) -> &Vec<HashSet<N>> {
        &self.max_cliques
    }

    fn pivot(&mut self, p: HashSet<N>) -> N {
        let pvec : Vec<N> = p.clone().into_iter().collect::<Vec<_>>();
        return pvec[pvec.clone().iter().
            map(| v | self.graph.neighbors(v.clone()).collect::<HashSet<N>>().
                intersection(&p).cloned().collect::<Vec<_>>().len()).
                collect::<Vec<_>>().iter().enumerate().
                map(|(x, y)| (y, x)).max().unwrap().1 as usize]
    }

    fn tomita(&mut self, p: HashSet<N>, r: HashSet<N>, x: HashSet<N>) {
        let mut p_fp = p.clone();
        let mut x_fp = x.clone();

        if p.is_empty() {
            if x.is_empty() {
                self.max_cliques.push(r.clone());
            }
            return;
        }

        let u = self.pivot(p);
        let u_neighbours = self.graph.neighbors(u.clone()).collect::<HashSet<N>>();
        let real_p = p_fp.difference(&u_neighbours).cloned().collect::<HashSet<N>>();

        for v in real_p.iter() {
            let v_neighbours = self.graph.neighbors(v.clone()).collect::<HashSet<N>>();

            let p_intersect_v_neighbors = p_fp.intersection(&v_neighbours).cloned().collect();
            let mut r_union_v = r.clone();
            r_union_v.insert(v.clone());
            let x_intersect_v_neighbors = x_fp.intersection(&v_neighbours).cloned().collect();

            self.tomita(p_intersect_v_neighbors, r_union_v, x_intersect_v_neighbors);

            p_fp.remove(v);
            x_fp.insert(*v);
        }

    }
}
