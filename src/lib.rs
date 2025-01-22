// interface of the crate

mod dist;
mod base;
mod convert;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use crate::base::{Graph, Matrix};

#[pyfunction]
fn floyd_warshall_u32<'a>(py: Python<'a>, edges: Matrix<u32>) -> Matrix<u32> {
    py.allow_threads(||{
        let g = Graph::from_edges(edges);
        dist::floyd_warshall(&g)
    })
}

#[pyfunction]
fn floyd_warshall<'a>(py: Python<'a>, edges: Bound<'a, PyDict>) -> Bound<'a, PyDict> {
    let (edges_r, i2o) = convert::from_tuples_matrix(edges);
    let raw_result = floyd_warshall_u32(py, edges_r);
    convert::into_tuples_matrix(py, raw_result, i2o)
}

#[pyfunction]
fn nearest_common_descendant_u32<'a>(py: Python<'a>, distances: Matrix<u32>, extreme: u32) -> Matrix<u32> {
    py.allow_threads(||{
        // we don't really use edges in the algorithm
        let g = Graph::from_dist(&distances);
        dist::nearest_common_descendant(&g, &distances, extreme)
    })
}

#[pyfunction]
fn nearest_common_descendant<'a>(py: Python<'a>, distances: Bound<'a, PyDict>, extreme: u32) -> Bound<'a, PyDict> {
    let (distances_r, i2o) = convert::from_tuples_matrix(distances);
    let raw_result = nearest_common_descendant_u32(py, distances_r, extreme);
    convert::into_tuples_matrix(py, raw_result, i2o)
}

#[pymodule]
fn graphs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(floyd_warshall_u32, m)?)?;
    m.add_function(wrap_pyfunction!(floyd_warshall, m)?)?;
    m.add_function(wrap_pyfunction!(nearest_common_descendant_u32, m)?)?;
    m.add_function(wrap_pyfunction!(nearest_common_descendant, m)?)?;
    Ok(())
}
