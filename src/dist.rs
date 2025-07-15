use std::collections::{HashMap, VecDeque};
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

/* more commonly known as lowest common ancestor in DAGs */
pub fn nearest_common_descendant_cubic<T>(graph: &Graph<T>, dist: &Matrix<T>, extreme: u32) -> Matrix<T>
where T: Hash, T: Copy, T: Eq {
    // NOTE we assume input `dist` matrix to be symmetric
    // TODO fix by assuming orientation!
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

pub fn nearest_common_descendant_quad<T>(graph: &Graph<T>, dist: &Matrix<T>, extreme: u32) -> Matrix<T>
where T: Hash, T: Copy, T: Eq, T: std::fmt::Debug {
    let mut remaining_children: HashMap<T, u32> = HashMap::new();
    let mut parents: HashMap<T, Vec<T>> = HashMap::new();
    let mut children: HashMap<T, Vec<T>> = HashMap::new();
    let mut queue: VecDeque<T> = VecDeque::new();
    let mut interm: HashMap<(T, T), (T, u32)> = HashMap::new();
    for v in graph.vertices.iter() {
        remaining_children.insert(*v, 0);
        parents.insert(*v, vec!());
        children.insert(*v, vec!());
    }
    for ((h, t), _) in graph.edges.iter() {
        remaining_children.entry(*h).and_modify(|v| *v+=1);
        parents.entry(*t).and_modify(|v| v.push(*h));
        children.entry(*h).and_modify(|v| v.push(*t));
    }
    remaining_children.iter().filter(|(_, c)| **c == 0).for_each(|(v, _)| queue.push_back(*v));

    while let Some(v) = queue.pop_front() {
        for u in graph.vertices.iter() {
            if let Some(dist) = dist.get(&(*u, v)) {
                interm.insert((v, *u), (v, *dist));
            }
            for c in children[&v].iter() {
                if let Some((d, _)) = interm.get(&(*c, *u)) {
                    let new_dist = max(dist[&(v, *d)], dist[&(*u, *d)]);
                    if let Some((_, old_dist)) = interm.get(&(v, *u)) {
                        if *old_dist > new_dist {
                            interm.insert((v, *u), (*d, new_dist));
                        }
                    } else {
                        interm.insert((v, *u), (*d, new_dist));
                    }
                }
            }
        }
        for p in parents[&v].iter() {
            let rc = remaining_children[p];
            if rc == 1 {
                queue.push_back(*p);
            } else {
                remaining_children.insert(*p, rc-1);
            }
        }
    }

    let mut ncd: Matrix<T> = HashMap::new();
    for v in graph.vertices.iter() {
        for u in graph.vertices.iter() {
            if let Some((_, dist)) = interm.get(&(*v, *u)) {
                ncd.insert((*v, *u), *dist);
            } else {
                ncd.insert((*v, *u), extreme);
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
        assert_eq!(nearest_common_descendant_cubic(&g2v, &dist, 42), ncd_dist_expected);
        assert_eq!(nearest_common_descendant_quad(&g2v, &dist, 42), ncd_dist_expected);
    }
}
