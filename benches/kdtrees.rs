use std::time::Duration;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn rand_2d() -> ([f32; 2], f32) {
    rand::random()
}

fn rand_3d() -> ([f32; 3], f64) {
    rand::random()
}

pub fn add_2d_kiddo(c: &mut Criterion) {
    let mut group = c.benchmark_group("kiddo 2d");

    for size in [100, 1_000, 10_000, 100_000, 1_000_000] {
        group.throughput(Throughput::Elements(size));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut points = vec![];
            for _ in 0..size {
                points.push(rand_2d());
            }

            let mut kdtree = kiddo::KdTree::with_per_node_capacity(16).unwrap();

            b.iter(|| {
                points.iter().for_each(|point| {
                    kdtree.add(&point.0, point.1);
                })
            });
        });
    }
}

pub fn add_3d_kiddo(c: &mut Criterion) {
    let mut group = c.benchmark_group("kiddo 3d");

    for size in [100, 1_000, 10_000, 100_000] {
        group.throughput(Throughput::Elements(size));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut points = vec![];
            for _ in 0..size {
                points.push(rand_3d());
            }

            let mut kdtree = kiddo::KdTree::with_per_node_capacity(16).unwrap();

            b.iter(|| {
                points.iter().for_each(|point| {
                    kdtree.add(&point.0, point.1);
                })
            });
        });
    }
}

pub fn add_2d_kdtree(c: &mut Criterion) {
    let mut group = c.benchmark_group("kdtree 2d");

    for size in [100, 1_000, 10_000, 100_000, 1_000_000] {
        group.throughput(Throughput::Elements(size));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut points = vec![];
            for _ in 0..size {
                points.push(rand_2d());
            }

            let mut kdtree = kdtree::KdTree::with_capacity(2, size as usize);

            b.iter(|| {
                points.iter().for_each(|point| {
                    kdtree.add(&point.0, point.1);
                })
            });
        });
    }
}

pub fn add_3d_kdtree(c: &mut Criterion) {
    let mut group = c.benchmark_group("kdtree 3d");

    for size in [100, 1_000, 10_000, 100_000] {
        group.throughput(Throughput::Elements(size));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut points = vec![];
            for _ in 0..size {
                points.push(rand_3d());
            }

            let mut kdtree = kdtree::KdTree::with_capacity(3, size as usize);

            b.iter(|| {
                points.iter().for_each(|point| {
                    kdtree.add(&point.0, point.1);
                })
            });
        });
    }
}

criterion_group!(name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(15));
    targets = add_2d_kiddo, add_3d_kiddo, add_2d_kdtree, add_3d_kdtree);
criterion_main!(benches);
