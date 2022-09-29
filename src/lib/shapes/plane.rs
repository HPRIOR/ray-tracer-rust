use crate::{
    geometry::vector::Tup,
    material::material::Material,
    matrix::matrix::Matrix,
    ray::ray::{Intersection, Ray},
};

use super::shape::TShape;

struct Plane;

impl TShape for Plane {
    fn material(&self) -> &Material {
        self.material()
    }

    fn transform(&self) -> &Matrix {
        self.transform()
    }

    fn normal_at(&self, point: Tup) -> Option<Tup> {
        todo!()
    }

    fn shape_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        todo!()
    }

    fn to_trait_ref(&self) -> Box<&dyn TShape> {
        Box::new(self)
    }

    fn shape_normal_at(&self, local_point: Tup) -> Tup {
        todo!()
    }
}
