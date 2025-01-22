use std::collections::HashMap;
use std::cmp::{Eq, max, min};
use std::hash::Hash;
use std::marker::Copy;

use crate::base::{Graph, Matrix};

pub fn floyd_warshall<T>(graph: &Graph<T>) -> Matrix<T>
where T: Hash, T: Copy, T: Eq {
    // let start = Instant::now();
    let mut dist = HashMap::new();
    for a in &graph.vertices {
        dist.insert((*a, *a), 0);
    }
    // let e1 = start.elapsed();
    for ((a, b), weight) in &graph.edges {
        dist.insert((*a, *b), *weight);
    }
    // let e2 = start.elapsed();

    for c in &graph.vertices {
        for a in &graph.vertices {
            for b in &graph.vertices {
                let e1 = &(*a, *c);
                let e2 = &(*c, *b);
                let et = &(*a, *b);
                if dist.contains_key(e1) && dist.contains_key(e2) {
                    let maybe = *dist.get(e1).unwrap() + *dist.get(e2).unwrap();
                    if dist.contains_key(et) {
                        if *dist.get(et).unwrap() > maybe {
                            dist.insert(*et, maybe);
                        }
                    } else {
                        dist.insert(*et, maybe);
                    }
                }
            }
        }
    }
    // let e3 = start.elapsed();

    /*
    println!("refl part: {}", e1.as_millis());
    println!("copy part: {}", e2.as_millis());
    println!("dist part: {}", e3.as_millis());
    */

    dist
}

pub fn nearest_common_descendant<T>(graph: &Graph<T>, dist: &Matrix<T>, extreme: u32) -> Matrix<T>
where T: Hash, T: Copy, T: Eq {
    // NOTE we assume input `dist` matrix to be symmetric
    // TODO fix? Or expose as a parameter?
    let mut ncd: Matrix<T> = HashMap::new();
    let vertices: Vec<&T> = graph.vertices.iter().collect();
    let n: usize = vertices.len();
    for a in &vertices {
        ncd.insert((**a, **a), 0);
    }

    // TODO this is like floyd-warshall, but there should be a better algorithm
    // NOTE we don't use the fw-iteration order (cab) but instead (abc) -- however both work
    for ai in 0..n {
        for bi in ai+1..n {
            let a = vertices[ai];
            let b = vertices[bi];
            for c in &graph.vertices {
                let m = *ncd.get(&(*a, *b)).unwrap_or(&extreme);
                let n = *max(dist.get(&(*a, *c)).unwrap_or(&extreme), dist.get(&(*b, *c)).unwrap_or(&extreme));
                let o = min(m, n);
                ncd.insert((*a, *b), o);
                ncd.insert((*b, *a), o);
            }
        }
    }
    ncd
}

#[cfg(test)]
mod tests {
    use crate::base::{reflexive, symmetric};
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn test_floyd_warshall() {
        let g2v = Graph {
            vertices: HashSet::from_iter(vec![0, 1, 2]),
            edges: symmetric(HashMap::from([
                ((0, 1), 1),
                ((1, 2), 1),
            ])),
        };
        let g2v_dist_expected = reflexive(symmetric(HashMap::from([
            ((0, 1), 1),
            ((0, 2), 2),
            ((1, 2), 1),
        ])));
        assert_eq!(floyd_warshall(&g2v), g2v_dist_expected);
    }

    #[test]
    fn test_nearest_common_descendant() {
        let g2v = Graph {
            vertices: HashSet::from_iter(vec![0, 1, 2, 3]),
            edges: HashMap::from([
                ((0, 2), 1),
                ((1, 2), 1),
                ((2, 3), 1),
            ]),
        };
        let dist = floyd_warshall(&g2v);
        println!("{:?}", dist);
        let ncd_dist_expected = symmetric(reflexive(HashMap::from([
            ((0, 1), 1),
            ((0, 2), 1),
            ((0, 3), 2),
            ((1, 2), 1),
            ((1, 3), 2),
            ((2, 3), 1),
        ])));
        assert_eq!(nearest_common_descendant(&g2v, &dist, 42), ncd_dist_expected);
    }
}
