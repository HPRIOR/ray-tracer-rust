#![allow(unused)]

use std::ops::Neg;

use crate::{
    geometry::vector::{vector, Tup},
    material::material::Material,
    matrix::matrix::Matrix,
    ray::ray::{Intersection, Ray},
};

use super::{shape::TShape, sphere::SphereBuilder};

pub struct PlaneBuilder {
    material: Material,
    transform: Matrix,
}

impl Default for PlaneBuilder {
    fn default() -> Self {
        Self {
            material: Default::default(),
            transform: Default::default(),
        }
    }
}

impl PlaneBuilder {
    pub fn new() -> Self {
        PlaneBuilder::default()
    }

    pub fn with_transform(mut self, matrix: Matrix) -> PlaneBuilder {
        self.transform = matrix;
        self
    }

    pub fn with_material(mut self, material: Material) -> PlaneBuilder {
        self.material = material;
        self
    }

    pub fn build(self) -> Plane {
        Plane {
            transform: self.transform,
            material: self.material,
        }
    }
    pub fn build_trait(self) -> Box<dyn TShape> {
        Box::new(Plane {
            transform: self.transform,
            material: self.material,
        })
    }
}

#[derive(Debug)]
pub struct Plane {
    material: Material,
    transform: Matrix,
}

impl Plane {
    pub fn builder() -> PlaneBuilder {
        PlaneBuilder::default()
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            material: Default::default(),
        }
    }
}

impl TShape for Plane {
    fn material(&self) -> &Material {
        &self.material
    }

    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn shape_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        if ray.direction.1.abs() <= 0.00001 {
            return vec![];
        };
        let t = ray.origin.1.neg() / ray.direction.1;
        return vec![Intersection::new(t, self.to_trait_ref())];
    }

    fn to_trait_ref(&self) -> Box<&dyn TShape> {
        Box::new(self)
    }

    fn shape_normal_at(&self, local_point: Tup) -> Tup {
        vector(0.0, 1.0, 0.0) // normal is constant for plane
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        geometry::vector::{point, vector},
        ray::ray::Ray,
        shapes::shape::TShape,
    };

    use super::Plane;

    #[test]
    fn normal_is_always_constant() {
        let p1 = Plane::default();
        let n1 = p1.normal_at(point(0.0, 0.0, 0.0));
        let n2 = p1.normal_at(point(10.0, 0.0, -10.0));
        let n3 = p1.normal_at(point(-5.0, 0.0, 150.0));

        assert_eq!(n1.unwrap(), vector(0.0, 1.0, 0.0));
        assert_eq!(n2.unwrap(), vector(0.0, 1.0, 0.0));
        assert_eq!(n3.unwrap(), vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn no_intersect_parallel_plane() {
        let p1 = Plane::default();
        let ray = Ray::new(point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = p1.shape_intersect(&ray);
        assert!(xs.is_empty())
    }
    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p1 = Plane::default();
        let ray = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = p1.shape_intersect(&ray);
        assert!(xs.is_empty())
    }

    #[test]
    fn intersect_from_above() {
        let p1 = Plane::default();
        let ray = Ray::new(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));
        let xs = p1.shape_intersect(&ray);
        assert_eq!(xs.len(), 1);
        let i = xs.get(0).unwrap();
        assert_eq!(i.at, 1.0);
        let object = *i.object.as_ref();
        assert!(std::ptr::eq(*i.object.as_ref(), *p1.to_trait_ref()));
    }

    #[test]
    fn intersect_from_below() {
        let p1 = Plane::default();
        let ray = Ray::new(point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs = p1.shape_intersect(&ray);
        assert_eq!(xs.len(), 1);
        let i = xs.get(0).unwrap();
        assert_eq!(i.at, 1.0);
        let object = *i.object.as_ref();
        assert!(std::ptr::eq(*i.object.as_ref(), *p1.to_trait_ref()));
    }
}
