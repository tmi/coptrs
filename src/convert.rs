// converting python objects to rust -- in some cases relying on pyo3 implicits doesnt seem to work
use std::collections::HashMap;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyDictMethods, PyTuple, PyTupleMethods};
use crate::base::Matrix;

pub fn from_tuples_matrix<'a>(distances: Bound<'a, PyDict>) -> (Matrix<u32>, HashMap<u32, Bound<'a, PyAny>>) {
    let mut distances_r = HashMap::new();
    let mut o2i = HashMap::new();
    let mut i2o = HashMap::new();
    let mut i: u32 = 0;
    for (key, value) in distances.iter() {
        let key_tuple: Bound<'a, PyTuple> = key.downcast_into().unwrap();
        let a = key_tuple.get_item(0).unwrap();
        let a_k = a.hash().unwrap();
        if !o2i.contains_key(&a_k) {
            o2i.insert(a_k, i);
            i2o.insert(i, a);
            i+=1;
        }
        let b = key_tuple.get_item(1).unwrap();
        let b_k = b.hash().unwrap();
        if !o2i.contains_key(&b_k) {
            o2i.insert(b_k, i);
            i2o.insert(i, b);
            i+=1;
        }
        let value_r = value.extract::<u32>().unwrap();
        let a_i: u32 = *o2i.get(&a_k).unwrap();
        let b_i: u32 = *o2i.get(&b_k).unwrap();
        distances_r.insert((a_i, b_i), value_r);
    }
    (distances_r, i2o)
}

pub fn into_tuples_matrix<'a>(py: Python<'a>, raw_result: Matrix<u32>, lookup: HashMap<u32, Bound<'a, PyAny>>) -> Bound<'a, PyDict> {
    let result = PyDict::new(py);
    for ((a, b), value) in raw_result.iter() {
        let key = PyTuple::new(py, [lookup.get(a).unwrap(), lookup.get(b).unwrap()]).unwrap();
        result.set_item(key, value).unwrap(); 
    }
    result
}
