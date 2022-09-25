use crate::{
    geometry::vector::Tup,
    material::material::Material,
    matrix::matrix::Matrix,
    ray::ray::{Ray, TIntersection},
};

pub trait TShape: Sync + Send{
    fn material(&self) -> &Material;
    fn transform(&self) -> &Matrix;
    fn normal_at(&self, point: Tup) -> Option<Tup>;
    fn local_intersect(&self, ray: &Ray) -> Vec<Box<dyn TIntersection>>;

    fn intersect(&self, ray: &Ray) -> Vec<Box<dyn TIntersection>> {
        let maybe_shape_transform = self.transform().inverse();
        if let Some(shape_tranform) = maybe_shape_transform {
            let local_ray = ray.transform(&shape_tranform);
            return self.local_intersect(&local_ray);
        }
        return vec![];
    }
}

pub trait TShapeBuilder: Sync + Send  {
    fn with_transform(self, matrix: Matrix) -> Self;
    fn with_material(self, material: Material) -> Self;
    fn build(self) -> Box<dyn TShape>;
}
