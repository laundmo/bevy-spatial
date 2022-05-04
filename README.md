# bevy_spatial

![](https://i.laundmo.com/tENe0/YIVuweru01.png)
from the `distance2d` example, colors the elements in a radius around the mouse.

A bevy plugin to track your entities in spatial indexes and query them.

Currently implemented features:
|feature|description|
|-|-|
|`kdtree`|KD-Tree for spatial lookups which is fully recreated. This is ideal for cases where EVERYTHING is moving.|
|`rstar`|R\*-Tree for spatial lookups which is updated or recreated based on a threshold of changes. Ideal when most entities are static. |

Quickstart using the `kdtree` feature:

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

## comaptible bey versions

| bevy | bevy_spatial |
| ---- | ------------ |
| 0.7  | 0.1.0        |

## TODOs and Ideas

- benchmarks
- documentation

- a feature for `linfa_nn` to use their abstractions over kdtree/balltree linear search
- a feature for https://github.com/InstantDomain/instant-distance
