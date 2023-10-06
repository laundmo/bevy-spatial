//! implementations to use [`kd_tree`] trees as a spatial datastructure in ``bevy_spatial``.

use bevy::prelude::*;
use kd_tree::{KdPoint, KdTree as BaseKdTree, KdTreeN};

use crate::{
    TComp,
    point::SpatialPoint,
    spatial_access::{SpatialAccess, UpdateSpatialAccess},
};

use std::marker::PhantomData;

use bevy::prelude::Resource;

#[cfg(all(feature = "kdtree_rayon", target_arch = "wasm32"))]
compile_error!(
    "bevy-spatial feature \"kdtree_rayon\" is incompatible with target_arch = \"wasm32\" builds. Disable default-features and enable kdtree"
);

macro_rules! kdtree_impl {
    ($pt:ty, $treename:ident) => {
        impl KdPoint for $pt {
            type Scalar = <$pt as SpatialPoint>::Scalar;

            type Dim = <$pt as SpatialPoint>::Dimension;

            fn at(&self, i: usize) -> Self::Scalar {
                <Self as SpatialPoint>::at(self, i)
            }
        }

        /// Resource for storing a ``KdTree``
        #[derive(Resource)]
        pub struct $treename<Comp> {
            /// The ``KdTree``
            pub tree: BaseKdTree<$pt>,
            component_type: PhantomData<Comp>,
        }

        impl<Comp> Default for $treename<Comp> {
            fn default() -> Self {
                Self {
                    tree: default(),
                    component_type: PhantomData,
                }
            }
        }

        impl<Comp> SpatialAccess for $treename<Comp>
        where
            Comp: TComp,
        {
            type Point = $pt;

            type Comp = Comp;
            type ResultT = (<$pt as SpatialPoint>::Vec, Option<Entity>);

            /// Get the nearest neighbour to a position.
            fn nearest_neighbour(&self, loc: <$pt as SpatialPoint>::Vec) -> Option<Self::ResultT> {
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
            ) -> Vec<Self::ResultT> {
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
            ) -> Vec<Self::ResultT> {
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

            /// Return all points which are within the specified rectangular axis-aligned region.
            /// loc1, loc2 are expected to be sorted along +X (+Y) +Z diagonal.
            fn within(
                &self,
                loc1: <Self::Point as SpatialPoint>::Vec,
                loc2: <Self::Point as SpatialPoint>::Vec,
            ) -> Vec<Self::ResultT> {
                let _span = info_span!("within").entered();

                let p1: $pt = loc1.into();
                let p2: $pt = loc2.into();

                let rect = [p1, p2];

                if self.tree.len() == 0 {
                    vec![]
                } else {
                    self.tree
                        .within(&rect)
                        .iter()
                        .map(|e| (e.vec(), e.entity()))
                        .collect()
                }
            }
        }
        impl<Comp: TComp> UpdateSpatialAccess for $treename<Comp> {
            fn update(
                &mut self,
                data: impl Iterator<Item = (Self::Point, bool)>,
                _: impl Iterator<Item = Entity>,
            ) {
                #[cfg(all(feature = "kdtree_rayon", not(target_arch = "wasm32")))]
                let tree =
                    KdTreeN::par_build_by_ordered_float(data.map(|(p, _)| p).collect::<Vec<_>>());
                #[cfg(any(not(feature = "kdtree_rayon"), target_arch = "wasm32"))]
                let tree =
                    KdTreeN::build_by_ordered_float(data.map(|(p, _)| p).collect::<Vec<_>>());
                self.tree = tree;
            }

            fn add(&mut self, _: Self::Point) {}

            fn remove_point(&mut self, _: Self::Point) -> bool {
                false
            }

            fn remove_entity(&mut self, _: Entity) -> bool {
                false
            }

            fn clear(&mut self) {
                self.tree = KdTreeN::default();
            }
        }
    };
}
kdtree_impl!(crate::point::Point2, KDTree2);
kdtree_impl!(crate::point::Point3, KDTree3);
kdtree_impl!(crate::point::Point3A, KDTree3A);
kdtree_impl!(crate::point::PointD2, KDTreeD2);
kdtree_impl!(crate::point::PointD3, KDTreeD3);
