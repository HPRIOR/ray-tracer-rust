#![allow(unused_variables, dead_code)]
use uuid::Uuid;

use crate::{
    geometry::vector::{point, Operations, Tup, Vector},
    material::material::Material,
    matrix::matrix::Matrix,
    ray::ray::{Intersection, Ray},
    utils::math_ext::Square,
};

use super::shape::{TShape, TShapeBuilder};

pub struct SphereBuilder {
    transform: Option<Matrix>,
    material: Option<Material>,
}

impl Default for SphereBuilder {
    fn default() -> Self {
        Self {
            transform: Some(Default::default()),
            material: Some(Default::default()),
        }
    }
}

impl TShapeBuilder for SphereBuilder {
    type ConcreteOutput = Sphere;
    type AbstractOutput = Box<dyn TShape>;

    fn with_transform(mut self, matrix: Matrix) -> Self {
        self.transform = Some(matrix);
        self
    }

    fn with_material(mut self, material: Material) -> Self {
        self.material = Some(material);
        self
    }

    fn build(self) -> Self::ConcreteOutput {
        Sphere {
            id: Uuid::new_v4(),
            transform: self.transform.unwrap_or(Matrix::ident()),
            material: self.material.unwrap_or(Material::default()),
        }
    }

    fn build_trait(self) -> Self::AbstractOutput {
        Box::new(Sphere {
            id: Uuid::new_v4(),
            transform: self.transform.unwrap_or(Matrix::ident()),
            material: self.material.unwrap_or(Material::default()),
        })
    }
}

#[derive(Debug)]
pub struct Sphere {
    pub id: Uuid,
    pub transform: Matrix,
    pub material: Material,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            id: Default::default(),
            transform: Default::default(),
            material: Default::default(),
        }
    }
}

impl Sphere {
    pub fn builder() -> SphereBuilder {
        SphereBuilder::default()
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_trait(&self) -> Box<&dyn TShape> {
        Box::new(self)
    }
}

impl TShape for Sphere {
    fn material(&self) -> &Material {
        &self.material
    }

    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn shape_normal_at(&self, local_point: Tup) -> Tup {
        local_point.sub(point(0.0, 0.0, 0.0))
    }

    fn shape_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let shape_to_ray = ray.origin.sub(point(0.0, 0.0, 0.0));

        let a = ray.direction.dot(ray.direction);
        let b = (ray.direction.dot(shape_to_ray)) * 2.0;
        let c = shape_to_ray.dot(shape_to_ray) - 1.0;

        // if negative then ray misses - no intersection
        let discriminant = b.squared() - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        let i1 = Intersection::new(t1, self.to_trait_ref());
        let i2 = Intersection::new(t2, self.to_trait_ref());
        vec![i1, i2]
    }

    fn to_trait_ref(&self) -> Box<&dyn TShape> {
        Box::new(self)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{
        geometry::vector::{point, vector},
        matrix::matrix::{Axis, Matrix},
        shapes::shape::{TShape, TShapeBuilder},
        utils::test::ApproxEq,
    };

    use super::Sphere;

    #[test]
    fn sphere_has_default_transformation() {
        let s = Sphere::new();
        assert_eq!(s.transform, Matrix::ident());
    }

    #[test]
    fn sphere_can_change_transformation() {
        let mut s = Sphere::new();
        let t = Matrix::translation(2.0, 3.0, 4.0);
        s.transform = t.clone();
        assert_eq!(s.transform, t);
    }

    #[test]
    fn sphere_can_be_created_with_new_transform() {
        let t = Matrix::translation(2.0, 3.0, 4.0);
        let s = Sphere::builder().with_transform(t.clone()).build();
        assert_eq!(s.transform, t);
    }

    #[test]
    fn normal_at_x_axis() {
        let s = Sphere::new();
        let sut = s.normal_at(point(1.0, 0.0, 0.0));
        assert_eq!(sut.unwrap(), vector(1.0, 0.0, 0.0))
    }
    #[test]
    fn normal_at_y_axis() {
        let s = Sphere::new();
        let sut = s.normal_at(point(0.0, 1.0, 0.0));
        assert_eq!(sut.unwrap(), vector(0.0, 1.0, 0.0))
    }

    #[test]
    fn normal_at_z_axis() {
        let s = Sphere::new();
        let sut = s.normal_at(point(0.0, 0.0, 1.0));
        assert_eq!(sut.unwrap(), vector(0.0, 0.0, 1.0))
    }

    #[test]
    fn normal_at_non_axial_point() {
        let s = Sphere::new();
        let sut = s.normal_at(point(
            (3.0_f64).sqrt() / 3.0,
            (3.0_f64).sqrt() / 3.0,
            (3.0_f64).sqrt() / 3.0,
        ));
        assert_eq!(
            sut.unwrap(),
            vector(
                (3.0_f64).sqrt() / 3.0,
                (3.0_f64).sqrt() / 3.0,
                (3.0_f64).sqrt() / 3.0,
            )
        )
    }

    #[test]
    fn normal_with_translated_sphere() {
        let s = Sphere::builder()
            .with_transform(Matrix::translation(0.0, 1.0, 0.0))
            .build();
        let sut = s.normal_at(point(0.0, 1.70711, -0.70711));
        sut.unwrap().approx_eq(vector(0.0, 0.70711, -0.70711))
    }

    #[test]
    fn normal_with_transformed_sphere() {
        let s = Sphere::builder()
            .with_transform(
                Matrix::ident()
                    .rotate(Axis::Z, PI / 5.0)
                    .scale(1.0, 0.5, 1.0),
            )
            .build();
        let sut = s.normal_at(point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));
        sut.unwrap().approx_eq(vector(0.0, 0.97014, -0.24254));
    }
}
