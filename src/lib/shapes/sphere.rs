#![allow(unused_variables, dead_code)]
use uuid::Uuid;

use crate::{
    geometry::vector::{point, Operations, Tup, Vector},
    material::material::Material,
    matrix::matrix::Matrix,
    ray::ray::{Ray, Intersection}
};

use super::shape::{TShape, TShapeBuilder};

pub struct SphereBuilder {
    transform: Option<Matrix>,
    material: Option<Material>,
}

impl Default for SphereBuilder {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            material: Default::default(),
        }
    }
}

impl SphereBuilder {
    fn new() -> Self {
        SphereBuilder::default()
    }
}

impl TShapeBuilder for SphereBuilder {
    fn with_transform(mut self, matrix: Matrix) -> SphereBuilder {
        self.transform = Some(matrix);
        self
    }

    fn with_material(mut self, material: Material) -> SphereBuilder {
        self.material = Some(material);
        self
    }

    fn build(self) -> Box<dyn TShape> {
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

pub enum As {
    Trait,
    Struct,
}

impl Sphere {
    pub fn builder() -> SphereBuilder {
        SphereBuilder::default()
    }

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

    pub fn with_attr(translation: Matrix, material: Material) -> Self {
        Self {
            id: Uuid::new_v4(),
            transform: translation,
            material,
        }
    }

    pub fn as_trait() -> Box<dyn TShape> {
        Box::new(Self {
            id: Uuid::new_v4(),
            transform: Matrix::ident(),
            material: Material::default(),
        })
    }

    pub fn as_trait_with_transform(translation: Matrix) -> Box<dyn TShape> {
        Box::new(Self {
            id: Uuid::new_v4(),
            transform: translation,
            material: Material::default(),
        })
    }

    pub fn as_trait_with_attr(translation: Matrix, material: Material) -> Box<dyn TShape> {
        Box::new(Self {
            id: Uuid::new_v4(),
            transform: translation,
            material,
        })
    }

    fn to_trait(&self) -> Box<&dyn TShape>{
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

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        todo!()
        // let new_ray = ray.transform(&self.transform());
        // let shape_to_ray = new_ray.origin.sub(point(0.0, 0.0, 0.0));
        //
        // let a = new_ray.direction.dot(new_ray.direction);
        // let b = (new_ray.direction.dot(shape_to_ray)) * 2.0;
        // let c = shape_to_ray.dot(shape_to_ray) - 1.0;
        //
        // // if negative then ray misses - no intersection
        // let discriminant = b.squared() - 4.0 * a * c;
        //
        // if discriminant < 0.0 {
        //     return vec![];
        // }
        //
        // let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        // let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        //
        //
        // todo!();
        // let i1 = Intersection::as_trait(t1, Box::new(self));
        // let i2 = Intersection::as_trait(t2, Box::new(self));
        // vec![i1, i2]
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{
        geometry::vector::{point, vector},
        matrix::matrix::{Axis, Matrix},
        shapes::shape::TShape,
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
