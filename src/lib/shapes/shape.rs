use crate::{geometry::vector::Tup, material::material::Material, matrix::matrix::Matrix};

pub trait TShape: Sync + Send {
    fn material(&self) -> &Material;
    fn transform(&self) -> &Matrix;
    fn normal_at(&self, point: Tup) -> Option<Tup>;
}
