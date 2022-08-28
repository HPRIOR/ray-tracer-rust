use crate::{geometry::vector::Tup, material::material::Material, matrix::matrix::Matrix};

pub trait HasNormal {
    fn normal_at(&self, point: Tup) -> Option<Tup>;
}

pub trait HasTransform {
    fn transform(&self) -> &Matrix;
}

pub trait HasMaterial {
    fn material(&self) -> &Material;
}

