use bevy::{
    math::{Vec2, Vec3, Vec3A},
    prelude::{Deref, DerefMut},
};
use kd_tree::KdPoint;
use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

#[derive(Deref, DerefMut, Clone, Copy)]
struct Vec2K(Vec2);

impl KdPoint for Vec2K {
    type Scalar = f32;
    type Dim = typenum::U2; // 2 dimensional tree.
    fn at(&self, k: usize) -> f32 {
        self.0[k]
    }
}
#[derive(Deref, DerefMut, Clone, Copy)]
struct Vec3K(Vec3);

impl KdPoint for Vec3K {
    type Scalar = f32;
    type Dim = typenum::U3; // 2 dimensional tree.
    fn at(&self, k: usize) -> f32 {
        self.0[k]
    }
}
#[derive(Deref, DerefMut, Clone, Copy)]
struct Vec3AK(Vec3A);

impl KdPoint for Vec3AK {
    type Scalar = f32;
    type Dim = typenum::U3; // 2 dimensional tree.
    fn at(&self, k: usize) -> f32 {
        self.0[k]
    }
}

fn rand_2d() -> Vec2K {
    Vec2K(Vec2::new(rand::random(), rand::random()))
}

fn rand_3d() -> Vec3K {
    Vec3K(Vec3::new(rand::random(), rand::random(), rand::random()))
}

fn rand_2din3d() -> Vec3K {
    Vec3K(Vec3::new(rand::random(), rand::random(), 0.0))
}

fn rand_3da() -> Vec3AK {
    Vec3AK(Vec3A::new(rand::random(), rand::random(), rand::random()))
}

fn rand_2din3da() -> Vec3AK {
    Vec3AK(Vec3A::new(rand::random(), rand::random(), 0.0))
}

pub fn compare_kdtrees(c: &mut Criterion) {
    let mut group = c.benchmark_group("kdtrees");

    for size in [1_000, 50_000, 100_000, 200_000, 500_000, 800_000, 1_000_000] {
        group.throughput(Throughput::Elements(size));
        group.bench_with_input(BenchmarkId::new("Vec2", size), &size, |b, &size| {
            let mut points = vec![];
            for _ in 0..size {
                points.push(rand_2d());
            }

            b.iter(|| kd_tree::KdTree::build_by_ordered_float(black_box(points.clone())));
        });
        group.bench_with_input(BenchmarkId::new("Vec3", size), &size, |b, &size| {
            let mut points = vec![];
            for _ in 0..size {
                points.push(rand_3d());
            }

            b.iter(|| kd_tree::KdTree::build_by_ordered_float(black_box(points.clone())));
        });
        group.bench_with_input(BenchmarkId::new("Vec2in3", size), &size, |b, &size| {
            let mut points = vec![];
            for _ in 0..size {
                points.push(rand_2din3d());
            }

            b.iter(|| kd_tree::KdTree::build_by_ordered_float(black_box(points.clone())));
        });
        group.bench_with_input(BenchmarkId::new("Vec3A", size), &size, |b, &size| {
            let mut points = vec![];
            for _ in 0..size {
                points.push(rand_3da());
            }

            b.iter(|| kd_tree::KdTree::build_by_ordered_float(black_box(points.clone())));
        });
        group.bench_with_input(BenchmarkId::new("Vec2in3A", size), &size, |b, &size| {
            let mut points = vec![];
            for _ in 0..size {
                points.push(rand_2din3da());
            }

            b.iter(|| kd_tree::KdTree::build_by_ordered_float(black_box(points.clone())));
        });
    }
}

criterion_group!(name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(15));
    targets = compare_kdtrees);
criterion_main!(benches);
