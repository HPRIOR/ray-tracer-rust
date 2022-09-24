use crate::{geometry::vector::Tup, material::material::Material, matrix::matrix::Matrix};

pub trait TShape: Sync + Send {
    fn material(&self) -> &Material;
    fn transform(&self) -> &Matrix;
    fn normal_at(&self, point: Tup) -> Option<Tup>;
}

pub trait TShapeBuilder: Sync + Send {
    fn with_transform(self, matrix: Matrix) ->  Self;
    fn with_material(self, material: Material) -> Self;
    fn build(self) -> Box<dyn TShape>;
}
