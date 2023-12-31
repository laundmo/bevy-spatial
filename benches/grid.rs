use divan::{black_box, Bencher};
use rand::prelude::*;
use simple_spatial_hash::FixedSizeGrid;

fn main() {
    divan::main();
}

const AMOUNTS: &[usize] = &[100000, 1000000, 5000000];

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

#[divan::bench(consts = AMOUNTS)]
fn update<const N: usize>(bencher: Bencher) {
    let setup = || {
        (
            FixedSizeGrid::new(glam::vec2(100., 100.), 10., glam::uvec2(10, 10)),
            gen_data(N, 100.),
        )
    };

    bencher
        .with_inputs(setup)
        .bench_local_values(|(mut grid, data)| grid.update(data.into_iter()))
}

const SIZES: &[i32] = &[10, 50, 100];

#[divan::bench(consts = SIZES)]
fn query_iter_i<const N: i32>(bencher: Bencher) {
    let setup = || {
        let mut grid = FixedSizeGrid::new(
            glam::vec2(N as f32, N as f32),
            N as f32 * 2.,
            glam::uvec2(N as u32, N as u32),
        );
        grid.update(gen_data(100000, N as f32 * 2.).into_iter());
        grid
    };

    bencher.with_inputs(setup).bench_local_values(|grid| {
        grid.get_in_radius(black_box(glam::vec2(5., 5.)), black_box(50.))
            .count();
    })
}
