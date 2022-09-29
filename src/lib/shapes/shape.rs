use crate::{
    geometry::vector::{Tup, Vector},
    material::material::Material,
    matrix::matrix::Matrix,
    ray::ray::{Intersection, Ray},
};

pub trait TShape: Sync + Send {
    fn material(&self) -> &Material;
    fn transform(&self) -> &Matrix;

    fn normal_at(&self, world_point: Tup) -> Option<Tup> {
        let maybe_local_normal = self
            .transform()
            .inverse()
            .map(|m| m.mul_tup(world_point))
            .map(|p| self.shape_normal_at(p)); // delegate to shape specific implementation

        let world_normal = maybe_local_normal.and_then(|object_norm| {
            self.transform()
                .transpose()
                .inverse()
                .map(|p| p.mul_tup(object_norm))
        });
        world_normal.map(|p| (p.0, p.1, p.2, 0.0).norm())
    }

    fn shape_normal_at(&self, local_point: Tup) -> Tup;

    fn shape_intersect(&self, ray: &Ray) -> Vec<Intersection>;

    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        // applies the shapes transform to the ray before passing this ray to the Shape specific
        // implementation of intersect 'shape_intersect'
        let maybe_shape_transform = self.transform().inverse();
        if let Some(shape_transform) = maybe_shape_transform {
            let local_ray = ray.transform(&shape_transform);
            return self.shape_intersect(&local_ray);
        }
        return vec![];
    }

    /// required to pass self to intersection, which must accept a reference to any shape
    fn to_trait_ref(&self) -> Box<&dyn TShape>;
}
