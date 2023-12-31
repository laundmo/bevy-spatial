use std::ops::Add;

use divan::Bencher;
use rand::prelude::*;
use simple_spatial_hash::SpatialHash2d;

fn main() {
    divan::main();
}

const PRECISIONS: &[i32] = &[-5, -4, -3, -2, -1, 1, 2, 3, 4, 5];

fn gen_data(n: usize, mult: f32) -> Vec<(glam::Vec2, usize)> {
    let mut data = Vec::with_capacity(n);
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        let x: f32 = rng.gen();
        let y: f32 = rng.gen();

        data.push((glam::vec2(x * mult, y * mult), rng.gen::<usize>()));
    }
    data
}

#[divan::bench(consts = PRECISIONS)]
fn add<const N: i32>(bencher: Bencher) {
    let setup = || (SpatialHash2d::new(N), gen_data(100000, 10.));

    bencher
        .with_inputs(setup)
        .bench_local_values(|(mut hash, data)| {
            for (pos, i) in data {
                hash.insert(pos, i)
            }
        })
}

#[divan::bench(consts = PRECISIONS)]
fn update_slight<const N: i32>(bencher: Bencher) {
    let setup = || {
        let mut hash = SpatialHash2d::new(N);
        let data = gen_data(1000, 10.);
        for (pos, i) in data.clone() {
            hash.insert(pos, i)
        }
        (
            hash,
            data.iter().map(|v| (v.0, v.0.add(0.1))).collect::<Vec<_>>(),
        )
    };

    bencher
        .with_inputs(setup)
        .bench_local_values(|(mut hash, moves)| {
            for (old, new) in moves {
                hash.update(old, new);
            }
        })
}

#[divan::bench(consts = PRECISIONS)]
fn update_random<const N: i32>(bencher: Bencher) {
    let setup = || {
        let mut hash = SpatialHash2d::new(N);
        let data = gen_data(100000, 10.);
        for (pos, i) in data.clone() {
            hash.insert(pos, i)
        }
        (
            hash,
            data.iter()
                .map(|v| v.0)
                .zip(gen_data(1000, 10.).iter().map(|v| v.0))
                .collect::<Vec<_>>(),
        )
    };

    bencher
        .with_inputs(setup)
        .bench_local_values(|(mut hash, moves)| {
            for (old, new) in moves {
                hash.update(old, new);
            }
        })
}
