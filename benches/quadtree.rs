use std::time::Duration;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use quadtree_rs::point::Point;

fn rand_2d() -> [i32; 2] {
    rand::random()
}

pub fn add_2d_quadtree(c: &mut Criterion) {
    let mut group = c.benchmark_group("quadtree 2d");

    for size in [100, 1_000, 10_000, 100_000, 1_000_000] {
        group.throughput(Throughput::Elements(size));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut points = vec![];
            for _ in 0..size {
                points.push(rand_2d());
            }

            let mut quadtree = quadtree_rs::Quadtree::<i32, usize>::new(8);

            b.iter(|| {
                points.iter().enumerate().for_each(|(i, point)| {
                    quadtree.insert_pt(
                        Point {
                            x: point[0],
                            y: point[1],
                        },
                        i,
                    );
                })
            });
        });
    }
}

criterion_group!(name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(5));
    targets = add_2d_quadtree);
criterion_main!(benches);
