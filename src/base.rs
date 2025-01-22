// base utility functions for graph building

// TODO expose some of this publicly?
// Symmetric/reflexive are useful but the conversion overhead probably not worth it. We would need some pipeline builder
// or a cross-language object first

use std::collections::{HashMap, HashSet};
use std::cmp::Eq;
use std::hash::Hash;
use std::marker::Copy;

pub type Matrix<T> = HashMap<(T, T), u32>;
#[derive(Debug, Eq, PartialEq)]
pub struct Graph<T>
where T: Hash, T: Eq {
    pub vertices: HashSet<T>,
    pub edges: Matrix<T>,
}

impl<T> Graph<T> where T: Hash, T: Eq, T: Copy {
    fn m2v(mat: &Matrix<T>) -> HashSet<T> {
        let mut vertices = HashSet::new();
        for ((a, b), _) in mat {
            vertices.insert(*a);
            vertices.insert(*b);
        }
        vertices
    }

    pub fn from_edges(edges: Matrix<T>) -> Self {
        Graph { vertices: Self::m2v(&edges), edges: edges}
    }

    pub fn from_dist(dist: &Matrix<T>) -> Self {
        Graph { vertices: Self::m2v(dist), edges: HashMap::new()}
    }
}

pub fn symmetric<T>(mut matrix: Matrix<T>) -> Matrix<T> 
where T: Hash, T: Copy, T: Eq {
    let keys = matrix.keys().map(|e| *e).collect::<Vec<_>>();
    for (a, b) in keys {
        if !matrix.contains_key(&(b, a)) {
            matrix.insert((b, a), *matrix.get(&(a, b)).unwrap());
        }
    }
    matrix
}

pub fn reflexive<T>(mut matrix: Matrix<T>) -> Matrix<T>
where T: Hash, T: Copy, T: Eq {
    let mut keys = HashSet::new();
    for (a, b) in matrix.keys() {
        if !keys.contains(a) { keys.insert(*a); }
        if !keys.contains(b) { keys.insert(*b); }
    }
    for key in keys {
        if !matrix.contains_key(&(key, key)) {
            matrix.insert((key, key), 0);
        }
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetric() {
        let m2v = HashMap::from([
            ((0, 1), 1),
            ((1, 2), 1),
            ((2, 1), 1),
        ]);
        let m2v_expected = HashMap::from([
            ((0, 1), 1),
            ((1, 0), 1),
            ((1, 2), 1),
            ((2, 1), 1),
        ]);
        assert_eq!(symmetric(m2v), m2v_expected);
    }

    #[test]
    fn test_reflexive() {
        let m2v = HashMap::from([
            ((0, 1), 1),
        ]);
        let m2v_expected = HashMap::from([
            ((0, 1), 1),
            ((0, 0), 0),
            ((1, 1), 0),
        ]);
        assert_eq!(reflexive(m2v), m2v_expected);
    }
}
