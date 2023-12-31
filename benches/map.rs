use std::hash::BuildHasherDefault;

use divan::Bencher;
use rand::prelude::*;
use rustc_hash::{FxHashMap, FxHasher};

fn main() {
    divan::main();
}

const LENGTHS: &[usize] = &[1, 10, 100, 1000, 10000, 100000];

#[divan::bench(consts = LENGTHS)]
fn sparse_vec<const N: usize>(bencher: Bencher) {
    let mut rng = rand::thread_rng();

    bencher
        .with_inputs(|| {
            (1..N)
                .map(|i| i * rng.gen_range(1..100))
                .collect::<Vec<_>>()
        })
        .bench_local_values(|vec| {
            let len = *vec.iter().max().unwrap_or(&0) + 1;
            let mut map = vec![0; len];
            for i in vec {
                map[i] = i;
            }
        })
}

#[divan::bench(consts = LENGTHS)]
fn fx_map<const N: usize>(bencher: Bencher) {
    let mut rng = rand::thread_rng();

    bencher
        .with_inputs(|| {
            (1..N)
                .map(|i| i * rng.gen_range(1..100))
                .collect::<Vec<_>>()
        })
        .bench_local_values(|vec| {
            let mut map = FxHashMap::with_capacity_and_hasher(
                vec.len(),
                BuildHasherDefault::<FxHasher>::default(),
            );
            for i in vec {
                map.insert(i, i);
            }
        })
}
