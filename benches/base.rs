use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use coptrs::base::{Graph, Matrix};
use std::collections::HashMap;
use rand::prelude::*;

fn edges_example(e: u32) -> Matrix<u32> {
    let mut hm = HashMap::with_capacity(e.try_into().unwrap());
    let n = e.isqrt(); // TODO parametrize with different densities / graph classes
    
    let mut rng = rand::rng();
    for _ in 0..e {
        let a = rng.next_u32() % n;
        let b = rng.next_u32() % n;
        hm.insert((a, b), 1);
    }
    hm
}
    
fn criterion_benchmark(c: &mut Criterion) {
    let s = 1000;
    c.bench_with_input(
        BenchmarkId::new("graph from edges", s),
        &s,
        |b, s| {
            let e = edges_example(*s);
            b.iter(|| Graph::from_edges(e.clone()));
        },
    );
}       
        
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
