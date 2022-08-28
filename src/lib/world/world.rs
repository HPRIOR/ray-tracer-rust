#![allow(unused_imports, unused_variables, dead_code)]
use crate::{
    colour::colour::Colour,
    geometry::vector::point,
    light::light::PointLight,
    material::material::Material,
    matrix::matrix::Matrix,
    ray::ray::{Ray, Intersection},
    shapes::{sphere::Sphere, shape::Shape},
};

struct World<T: Shape> {
    pub objects: Vec<T>,
    pub light: PointLight,
}

impl<'a, T:Shape> World<T> {
    pub fn intersect_world(&'a self, ray: &'a Ray) -> Vec<Intersection<'a, T>> {
        let mut result: Vec<Intersection<'a, T>> = self
            .objects
            .iter()
            .flat_map(|o| 
                ray.intersect(o)
            )
            .collect();
        result.sort_by(|a, b| a.at.total_cmp(&b.at));
        result
    }
}

impl Default for World<Sphere> {
    fn default() -> Self {
        let s1 = Sphere::with_attributes(
            Matrix::ident(),
            Material::new(0.1, 0.7, 0.2, 200.0, Colour::new(0.8, 1.0, 0.6)),
        );
        let s2 = Sphere::with_transform(Matrix::scaling(0.5, 0.5, 0.5));
        Self {
            objects: vec![s1, s2],
            light: PointLight::new(point(-10.0, 10.0, -10.0), Colour::white()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        colour::colour::Colour,
        geometry::vector::{point, vector},
        matrix::matrix::Matrix,
        ray::ray::Ray,
    };

    use super::World;

    #[test]
    fn default_world() {
        let world = World::default();
        assert_eq!(world.objects.len(), 2);
        assert_eq!(world.light.intensity, Colour::white());
        assert_eq!(world.light.position, point(-10.0, 10.0, -10.0));
        let s1 = &world.objects[0];
        let s2 = &world.objects[1];

        assert_eq!(s1.material.colour, Colour::new(0.8, 1.0, 0.6));
        assert_eq!(s1.material.diffuse, 0.7);
        assert_eq!(s1.material.specular, 0.2);
        assert_eq!(s2.transform, Matrix::scaling(0.5, 0.5, 0.5));
    }

    #[test]
    fn can_get_world_intersects() {
        let world = World::default();
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let sut = world.intersect_world(&ray);
        assert_eq!(sut.len(), 4);
        assert_eq!(sut[0].at, 4.0);
        assert_eq!(sut[1].at, 4.5);
        assert_eq!(sut[2].at, 5.5);
        assert_eq!(sut[3].at, 6.0);
    }
}
