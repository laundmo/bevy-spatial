use bevy::prelude::*;

#[derive(Debug)]
struct A {
    v: Vec2,
}

impl From<Vec2> for A {
    fn from(v: Vec2) -> Self {
        A { v }
    }
}

fn main() {
    let vec = Vec3::ZERO;
    let a: A = vec.into();
    println!("{}", a);
}
