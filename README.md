# bevy_spatial

<p align="center">
    <img src="https://i.laundmo.com/tENe0/rozUsOnA55.png/raw">
</p>

A bevy plugin to track your entities in spatial indices and query them.

![crates.io](https://img.shields.io/crates/v/bevy_spatial.svg)

Currently implemented features:
| Feature            | Description                                                                                                          |
| ------------------ | -------------------------------------------------------------------------------------------------------------------- |
| `kdtree` (default) | KD-Tree for spatial lookups which is fully recreated on update, but fast to recreate. Works well in most situations. |

```rust
use bevy_spatial::{AutomaticUpdate, KDTree3, TransformMode, SpatialAccess};

#[derive(Component, Default)]
struct TrackedByKDTree;

fn main() {
    App::new()
        .add_plugins(AutomaticUpdate::<TrackedByKDTree>::new()
            .with_frequency(Duration::from_secs_f32(0.3))
            .with_transform(TransformMode::GlobalTransform))
        .add_systems(Update, use_neighbour);
    // ...
}

type NNTree = KDTree3<TrackedByKDTree>; // type alias for later

// spawn some entities with the TrackedByKDTree component

fn use_neighbour(tree: Res<NNTree>){
    if let Some((pos, entity)) = tree.nearest_neighbour(Vec3::ZERO) {
        // pos: Vec3
        // do something with the nearest entity here
    }
}
```

For more details on usage see [Examples](https://github.com/laundmo/bevy-spatial/tree/main/examples)

## compatible bevy versions

| bevy | bevy_spatial |
| ---- | ------------ |
| 0.14 | 0.9.0        |
| 0.13 | 0.8.0        |
| 0.12 | 0.7.0        |
| 0.11 | 0.6.0        |
| 0.10 | 0.5.0        |
| 0.9  | 0.4.0        |
| 0.8  | 0.3.0        |
| 0.8  | 0.2.1        |
| 0.7  | 0.1          |

wasm caveats: the rayon acceleration for kdtree is disabled on wasm, making it a bit slower.
