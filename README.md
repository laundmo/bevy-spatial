# bevy_spatial

A bevy plugin to track your entities in spatial indexes and query them.

Qucikstart using the `kdtree` feature:

```rust
use bevy_spatial::{KDTreeAccess2D, KDTreePlugin2D, SpatialAccess};

#[derive(Component)]
struct TrackedByKDTree;

fn main() {
   App::new()
       .add_plugin(KDTreePlugin2D::<TrackedByKDTree> { ..default() })
   // ...
}

type NNTree = KDTreeAccess2D<TrackedByKDTree>; // type alias for later

fn (tree: Res<NNTree>){
    if let Some((pos, entity)) = treeaccess.nearest_neighbour(Vec2::ZERO) {
        // pos: Vec3
        // do something with the nearest entity here
    }
}
```

For more details on usage see [Examples](https://github.com/laundmo/bevy-spatial/tree/main/examples)

## TODOs and Ideas

- benchmarks
- documentation

- a feature for `linfa_nn` to use their abstractions over kdtree/balltree linear search
- a feature for https://github.com/InstantDomain/instant-distance
