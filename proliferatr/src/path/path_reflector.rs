use std::marker::PhantomData;

use super::{PathMutator, PointPath};

pub trait Reflection {
    fn reflect<P: PointPath>(path: &mut P);
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct XAxis;

impl Reflection for XAxis {
    fn reflect<P: PointPath>(path: &mut P) {
        path.points_mut().for_each(|p| p.reflect_x_mut());
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct YAxis;

impl Reflection for YAxis {
    fn reflect<P: PointPath>(path: &mut P) {
        path.points_mut().for_each(|p| p.reflect_y_mut());
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BothAxis;

impl Reflection for BothAxis {
    fn reflect<P: PointPath>(path: &mut P) {
        path.points_mut().for_each(|p| {
            p.reflect_x_mut();
            p.reflect_y_mut();
        });
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathReflector<T: Reflection> {
    _axis: PhantomData<T>,
}

impl<T: Reflection> PathMutator for PathReflector<T> {
    fn mutate<P: PointPath>(&mut self, path: &mut P) -> bool {
        T::reflect(path);
        true
    }
}

pub type XAxisReflector = PathReflector<XAxis>;
pub type YAxisReflector = PathReflector<YAxis>;
pub type BothAxisReflector = PathReflector<BothAxis>;
