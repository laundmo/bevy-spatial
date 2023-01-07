use bevy::{
    math::{Vec2, Vec3, Vec3A},
    prelude::{Deref, DerefMut},
};
use rstar::{DefaultParams, PointDistance, RTree, RTreeObject, AABB};
use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

#[derive(Deref, DerefMut, Clone, Copy)]
struct Vec2K(Vec2);

impl RTreeObject for Vec2K {
    type Envelope = AABB<[f32; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point(self.0.into())
    }
}

impl PointDistance for Vec2K {
    fn distance_2(&self, point: &[f32; 2]) -> f32 {
        self.0.distance_squared(Vec2::from_slice(point))
    }
}

#[derive(Deref, DerefMut, Clone, Copy)]
struct Vec3K(Vec3);

impl RTreeObject for Vec3K {
    type Envelope = AABB<[f32; 3]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point(self.0.into())
    }
}

impl PointDistance for Vec3K {
    fn distance_2(&self, point: &[f32; 3]) -> f32 {
        self.distance_squared(Vec3::from_slice(point))
    }
}

#[derive(Deref, DerefMut, Clone, Copy)]
struct Vec3AK(Vec3A);

impl RTreeObject for Vec3AK {
    type Envelope = AABB<[f32; 3]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point(self.0.into())
    }
}

impl PointDistance for Vec3AK {
    fn distance_2(&self, point: &[f32; 3]) -> f32 {
        self.distance_squared(Vec3A::from_slice(point))
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

pub fn compare_rstar(c: &mut Criterion) {
    let mut group = c.benchmark_group("rstartrees");

    for size in [1_000, 50_000, 100_000, 200_000, 500_000, 800_000, 1_000_000] {
        group.throughput(Throughput::Elements(size));
        group.bench_with_input(BenchmarkId::new("Vec2", size), &size, |b, &size| {
            let mut points = vec![];
            for _ in 0..size {
                points.push(rand_2d());
            }

            b.iter(|| {
                RTree::<Vec2K, DefaultParams>::bulk_load_with_params(black_box(points.clone()))
            })
        });
        group.bench_with_input(BenchmarkId::new("Vec3", size), &size, |b, &size| {
            let mut points = vec![];
            for _ in 0..size {
                points.push(rand_3d());
            }

            b.iter(|| {
                RTree::<Vec3K, DefaultParams>::bulk_load_with_params(black_box(points.clone()))
            })
        });
        group.bench_with_input(BenchmarkId::new("Vec2in3", size), &size, |b, &size| {
            let mut points = vec![];
            for _ in 0..size {
                points.push(rand_2din3d());
            }

            b.iter(|| {
                RTree::<Vec3K, DefaultParams>::bulk_load_with_params(black_box(points.clone()))
            })
        });
        group.bench_with_input(BenchmarkId::new("Vec3A", size), &size, |b, &size| {
            let mut points = vec![];
            for _ in 0..size {
                points.push(rand_3da());
            }

            b.iter(|| {
                RTree::<Vec3AK, DefaultParams>::bulk_load_with_params(black_box(points.clone()))
            })
        });
        group.bench_with_input(BenchmarkId::new("Vec2in3A", size), &size, |b, &size| {
            let mut points = vec![];
            for _ in 0..size {
                points.push(rand_2din3da());
            }

            b.iter(|| {
                RTree::<Vec3AK, DefaultParams>::bulk_load_with_params(black_box(points.clone()))
            })
        });
    }
}

criterion_group!(name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(15));
    targets = compare_rstar);
criterion_main!(benches);
