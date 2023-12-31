use divan::{black_box, Bencher};
use simple_spatial_hash::FixedSizeGrid;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_index(bencher: Bencher) {
    let setup = || FixedSizeGrid::new(glam::vec2(100., 100.), 10., glam::uvec2(10, 10));

    bencher
        .with_inputs(setup)
        .bench_local_values(|grid: FixedSizeGrid<usize>| {
            grid.get_index_clamped(black_box(glam::vec2(5., 5.)))
        })
}
#[divan::bench]
fn bench_map(bencher: Bencher) {
    let setup = || FixedSizeGrid::new(glam::vec2(100., 100.), 10., glam::uvec2(10, 10));

    bencher
        .with_inputs(setup)
        .bench_local_values(|grid: FixedSizeGrid<usize>| {
            grid.get_mapped_point(black_box(glam::vec2(5., 5.)))
        })
}
