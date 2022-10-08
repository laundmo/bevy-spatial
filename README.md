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
|`rstar`|R\*-Tree for spatial lookups which is updated or recreated based on a threshold of changed entities. Ideal when most entities are static. |

Quickstart using the `kdtree` feature:

```rust
use bevy_spatial::{KDTreeAccess2D, KDTreePlugin2D, SpatialAccess};

#[derive(Component)]
struct TrackedByKDTree;

fn main() {
   App::new()
       .add_plugin(KDTreePlugin2D::<TrackedByKDTree> { ..default() })
       .add_system(use_neighbour);
   // ...
}

type NNTree = KDTreeAccess2D<TrackedByKDTree>; // type alias for brevity

fn use_neighbour(tree: Res<NNTree>){
    if let Some((pos, entity)) = tree.nearest_neighbour(Vec2::ZERO) {
        // pos: Vec3
        // do something with the nearest entity here, most likley you will want a `query.get(entity)` call
    }
}
```

For more details on usage see [Examples](https://github.com/laundmo/bevy-spatial/tree/main/examples)

## compatible bevy versions

| bevy | bevy_spatial |
| ---- | ------------ |
| 0.8  | 0.3.0        |
| 0.8  | 0.2.1        |
| 0.7  | 0.1          |

wasm caveats: the rayon acceleration for kdtree is disabled on wasm, making it a bit slower.

## TODOs and Ideas

- benchmarks
- documentation

- Versions of the SpatialAccess functions which return Iterators instead of Vecs

- a feature for `linfa_nn` to use their abstractions over kdtree/balltree linear search
- a feature for https://github.com/InstantDomain/instant-distance
