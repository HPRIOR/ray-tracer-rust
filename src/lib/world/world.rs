#![allow(unused_imports, unused_variables, dead_code)]
use crate::{
    colour::colour::Colour,
    geometry::vector::{point, Operations, Tup, Vector},
    light::light::PointLight,
    material::material::Material,
    matrix::matrix::Matrix,
    ray::ray::{Intersection, PreComp, Ray, TIntersection},
    shapes::{shape::TShape, sphere::Sphere},
};

/*
 * The structure of the code seems disjointed. The key structures are Ray, Intersection, Shape,
 * Light, Material and World. The interface for World is good - it defines a world with some object
 * and a light - and a method from ray -> colour. Ray is messy. It can returns intersections when
 * passed objects. And then can be passed back these intersections to calculate pre-computations.
 * There is a cyclic dependency between them. Need to think of a way to organise the code to stop
 * this. There are other dependencies like this too.
 * If intersect world does not need to be used by anything else in World, then Ray should
 * encapsulate the retrieval of intersections, and just return pre-comps.
 *
 *
 */

struct World {
    pub objects: Vec<Box<dyn TShape>>,
    pub light: PointLight,
}

impl<'a> World {
    pub fn color_at(&'a self, ray: &'a Ray) -> Colour {
        let intersections: Vec<Box<dyn TIntersection<'a> + 'a>> =
            ray.intersect_objects(&self.objects);
        let maybe_intersection = intersections.get(0);

        let maybe_shade_hit = maybe_intersection
            .and_then(|i| ray.prepare_computations(i))
            .map(|pc| pc.shade_hit(&self.light));

        if let Some(shade_hit) = maybe_shade_hit {
            shade_hit
        } else {
            Colour::black()
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let s1 = Sphere::as_trait_with_attr(
            Matrix::ident(),
            Material::new(0.1, 0.7, 0.2, 200.0, Colour::new(0.8, 1.0, 0.6)),
        );
        let s2 = Sphere::as_trait_with_transform(Matrix::scaling(0.5, 0.5, 0.5));
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
        light::light::PointLight,
        matrix::matrix::Matrix,
        ray::ray::{Intersection, Ray},
        utils::test::ApproxEq,
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

        assert_eq!(s1.material().colour, Colour::new(0.8, 1.0, 0.6));
        assert_eq!(s1.material().diffuse, 0.7);
        assert_eq!(s1.material().specular, 0.2);
        assert_eq!(s2.transform(), &Matrix::scaling(0.5, 0.5, 0.5));
    }

    #[test]
    fn can_get_world_intersects() {
        let world = World::default();
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let sut = ray.intersect_objects(&world.objects);
        assert_eq!(sut.len(), 4);
        assert_eq!(sut[0].at(), 4.0);
        assert_eq!(sut[1].at(), 4.5);
        assert_eq!(sut[2].at(), 5.5);
        assert_eq!(sut[3].at(), 6.0);
    }
    #[test]
    fn shading_at_intersection_is_correct_from_outside() {
        let w = World::default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = &w.objects[0];
        let i = Intersection::as_trait(4.0, shape);
        let comp = r.prepare_computations(&i).unwrap();
        let c = comp.shade_hit(&w.light);
        c.approx_eq(Colour::new(0.38066, 0.47583, 0.2855));
    }
    #[test]
    fn shading_at_intersection_is_correct_from_inside() {
        let mut w = World::default();
        w.light = PointLight::new(point(0.0, 0.25, 0.0), Colour::white());
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let i = Intersection::as_trait(0.5, shape);
        let comp = r.prepare_computations(&i).unwrap();
        let c = comp.shade_hit(&w.light);
        c.approx_eq(Colour::new(0.90498, 0.90498, 0.90498));
    }
}
