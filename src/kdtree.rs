use bevy::prelude::*;
use kd_tree::{KdPoint, KdTree as BaseKdTree, KdTreeN};

use crate::{
    point::{SpatialPoint, SpatialTracker},
    spatial_access::{SpatialAccess, UpdateSpatialAccess},
    TComp,
};

use std::marker::PhantomData;

use bevy::prelude::Resource;

macro_rules! kdtree_impl {
    ($pt:ty, $treename:ident, $pluginname:ident) => {
        impl KdPoint for $pt {
            type Scalar = <$pt as SpatialPoint>::Scalar;

            type Dim = <$pt as SpatialPoint>::Dimension;

            fn at(&self, i: usize) -> Self::Scalar {
                <Self as SpatialPoint>::at(self, i)
            }
        }

        #[derive(Resource, Default)]
        pub struct $treename<Comp> {
            pub tree: BaseKdTree<$pt>,
            pub component_type: PhantomData<Comp>,
        }

        #[derive(Default)]
        pub struct $pluginname<Comp> {
            tcomp: PhantomData<Comp>,
        }

        impl<Comp> SpatialAccess for $treename<Comp>
        where
            Comp: TComp,
        {
            type Point = $pt;

            type Comp = Comp;
            type ResultT = Vec<(<$pt as SpatialPoint>::Vec, Option<Entity>)>;

            /// Get the nearest neighbour to a position.
            fn nearest_neighbour(
                &self,
                loc: <$pt as SpatialPoint>::Vec,
            ) -> Option<(<$pt as SpatialPoint>::Vec, Option<Entity>)> {
                let p: $pt = loc.into();
                let res = self.tree.nearest(&p);
                res.map(|point| (point.item.vec(), point.item.entity()))
            }

            /// Get the `k` neighbours to `loc`
            ///
            /// If `loc` is the location of a tracked entity, you might want to skip the first.
            fn k_nearest_neighbour(
                &self,
                loc: <$pt as SpatialPoint>::Vec,
                k: usize,
            ) -> Self::ResultT {
                let _span = info_span!("k-nearest").entered();
                let p: $pt = loc.into();

                self.tree
                    .nearests(&p, k)
                    .iter()
                    .map(|e| (e.item.vec(), e.item.entity()))
                    .collect()
            }

            /// Get all entities within a certain distance (radius) of `loc`
            fn within_distance(
                &self,
                loc: <$pt as SpatialPoint>::Vec,
                distance: <$pt as SpatialPoint>::Scalar,
            ) -> Self::ResultT {
                let _span = info_span!("within-distance").entered();

                let distance: <$pt as KdPoint>::Scalar = distance.into();

                if self.tree.len() == 0 {
                    vec![]
                } else {
                    let p: $pt = loc.into();

                    self.tree
                        .within_radius(&p, distance)
                        .iter()
                        .map(|e| (e.vec(), e.entity()))
                        .collect()
                }
            }
        }
        impl<Comp: TComp> UpdateSpatialAccess for $treename<Comp> {
            fn rebuild(&mut self, data: impl Iterator<Item = Self::Point>) {
                self.tree = KdTreeN::build_by_ordered_float(data.collect::<Vec<_>>());
            }

            fn add(&mut self, _: Self::Point) {}

            fn remove_point(&mut self, _: Self::Point) -> bool {
                false
            }

            fn remove_entity(&mut self, _: Entity) -> bool {
                false
            }

            fn clear(&mut self) {}
        }
    };
}
kdtree_impl!(crate::point::Point2, KDTree2, KDTreePlugin2);
kdtree_impl!(crate::point::Point3, KDTree3, KDTreePlugin3);
kdtree_impl!(crate::point::Point3A, KDTree3A, KDTreePlugin3A);
kdtree_impl!(crate::point::PointD2, KDTreeD2, KDTreePluginD2);
kdtree_impl!(crate::point::PointD3, KDTreeD3, KDTreePluginD3);

impl<Comp: TComp> Plugin for KDTreePlugin2<Comp> {
    fn build(&self, app: &mut App) {
        fn update<Comp: TComp>(
            tree: Res<KDTree2<Comp>>,
            query: Query<&SpatialTracker<crate::point::Point2, Comp>>,
        ) {
            tree.rebuild(query.iter().map(|i| i.point));
        }
        app.add_system(update)
    }
}
