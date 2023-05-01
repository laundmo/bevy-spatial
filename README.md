# bevy_spatial

<p align="center">
    <img src="https://i.laundmo.com/tENe0/rozUsOnA55.png/raw">
</p>

A bevy plugin to track your entities in spatial indices and query them.

![crates.io](https://img.shields.io/crates/v/bevy_spatial.svg)

NOTE: You will need to enable at least one of the features.

Currently implemented features:
|Feature|Description|
|-|-|
|`kdtree`|KD-Tree for spatial lookups which is fully recreated. This is ideal for cases where most entities are moving.|

Quickstart using the `kdtree` feature:

```rust
use bevy_spatial::{Spatial, KDTree3, SpatialAccess};

#[derive(Component, Default)]
struct TrackedByKDTree;

fn main() {
   App::new()
       .add_plugin(AutomaticUpdate::new::<TrackedByKDTree>()
               .spatial_structure(SpatialStructure::KDTree3)
               .update_automatic_with(Duration::from_secs(1), TransformMode::Transform))
       .add_system(use_neighbour);
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
| 0.10 | 0.5.0        |
| 0.9  | 0.4.0        |
| 0.8  | 0.3.0        |
| 0.8  | 0.2.1        |
| 0.7  | 0.1          |

wasm caveats: the rayon acceleration for kdtree is disabled on wasm, making it a bit slower.
