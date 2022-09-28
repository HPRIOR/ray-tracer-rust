use crate::{
    geometry::vector::Tup,
    material::material::Material,
    matrix::matrix::Matrix,
    ray::ray::{Ray, Intersection},
};

pub trait TShape: Sync + Send{
    fn material(&self) -> &Material;
    fn transform(&self) -> &Matrix;
    fn normal_at(&self, point: Tup) -> Option<Tup>;
    fn shape_intersect(&self, ray: &Ray) -> Vec<Intersection>;

    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let maybe_shape_transform = self.transform().inverse();
        if let Some(shape_transform) = maybe_shape_transform {
            let local_ray = ray.transform(&shape_transform);
            return self.shape_intersect(&local_ray);
        }
        return vec![];
    }

    fn to_trait_ref(&self) -> Box<&dyn TShape>;
}

