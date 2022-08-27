#![allow(unused_variables, dead_code)]
use uuid::Uuid;

use crate::{
    geometry::vector::{point, Operations, Tup, Vector},
    material::material::Material,
    matrix::matrix::Matrix,
};

use super::shape::{HasMaterial, HasNormal, HasTransform, IsShape};

#[derive(Debug)]
pub struct Sphere {
    pub id: Uuid,
    pub transform: Matrix,
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            transform: Matrix::ident(),
            material: Material::default(),
        }
    }

    pub fn with_transform(translation: Matrix) -> Self {
        Self {
            id: Uuid::new_v4(),
            transform: translation,
            material: Material::default(),
        }
    }

    pub fn with_attributes(translation: Matrix, material: Material) -> Self {
        Self {
            id: Uuid::new_v4(),
            transform: translation,
            material,
        }
    }
}

impl IsShape for Sphere{}

impl HasNormal for Sphere {
    fn normal_at(&self, world_point: Tup) -> Option<Tup> {
        let object_normal = self
            .transform
            .inverse()
            .map(|p| p.mul_tup(world_point).sub(point(0.0, 0.0, 0.0)));

        let world_normal = object_normal.and_then(|object_norm| {
            self.transform
                .transpose()
                .inverse()
                .map(|p| p.mul_tup(object_norm))
        });
        world_normal.map(|p| (p.0, p.1, p.2, 0.0).norm())
    }
}

impl HasTransform for Sphere {
    fn transform(&self) -> &Matrix {
        &self.transform
    }
}

impl HasMaterial for Sphere {
    fn material(&self) -> &Material {
        &self.material
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{
        geometry::vector::{point, vector},
        matrix::matrix::{Axis, Matrix},
        shapes::shape::HasNormal,
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
        let s = Sphere::with_transform(t.clone());
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
    fn normal_at_nonaxial_point() {
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
        let s = Sphere::with_transform(Matrix::translation(0.0, 1.0, 0.0));
        let sut = s.normal_at(point(0.0, 1.70711, -0.70711));
        sut.unwrap().approx_eq(vector(0.0, 0.70711, -0.70711))
    }

    #[test]
    fn normal_with_transformed_sphere() {
        let s = Sphere::with_transform(
            Matrix::ident()
                .rotate(Axis::Z, PI / 5.0)
                .scale(1.0, 0.5, 1.0),
        );
        let sut = s.normal_at(point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));
        sut.unwrap().approx_eq(vector(0.0, 0.97014, -0.24254));
    }
}
