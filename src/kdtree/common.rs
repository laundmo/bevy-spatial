use std::marker::PhantomData;

use crate::point::SpatialPoint;
use bevy::prelude::{Resource, Vec3};
use kd_tree::{KdPoint, KdTree as BaseKdTree};

#[derive(Resource)]
pub struct KDTree<TComp, KDItem>
where
    KDItem: KdPoint,
{
    pub tree: BaseKdTree<KDItem>,
    pub component_type: PhantomData<TComp>,
}

macro_rules! kdtree_impl {
    ($pt:ty) => {
        impl KdPoint for $pt {
            type Scalar = <$pt as SpatialPoint>::Scalar;

            type Dim = <$pt as SpatialPoint>::Dimension;

            fn at(&self, i: usize) -> Self::Scalar {
                <Self as SpatialPoint>::at(self, i)
            }
        }
    };
}
kdtree_impl!(crate::point::Point2);
kdtree_impl!(crate::point::Point3);
kdtree_impl!(crate::point::Point3A);
kdtree_impl!(crate::point::PointD2);
kdtree_impl!(crate::point::PointD3);
