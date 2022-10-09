#![allow(unused_imports, unused_variables, dead_code)]
use crate::{
    colour::colour::Colour,
    geometry::vector::{point, Operations, Tup, Vector},
    light::light::PointLight,
    material::material::Material,
    matrix::matrix::Matrix,
    ray::ray::{Hit, Intersection, PreComp, Ray},
    shapes::{shape::TShape, sphere::Sphere},
};

pub struct World {
    pub objects: Vec<Box<dyn TShape>>,
    pub light: PointLight,
}

impl World {
    pub fn new(objects: Vec<Box<dyn TShape>>, light: PointLight) -> Self {
        Self { objects, light }
    }

    pub fn color_at(&self, ray: &Ray, ref_lim: u32) -> Colour {
        let intersections: Vec<Intersection> = ray.intersect_objects(&self.objects);

        let maybe_intersection = intersections.hit();

        let maybe_precomp = maybe_intersection.and_then(|i| ray.prep_comps(i));

        let is_shadowed = maybe_precomp
            .as_ref()
            .map(|pc| self.is_shadowed(pc.over_point))
            .unwrap_or(false);

        if is_shadowed {
            return Colour::black();
        }

        // passing is shadow into shade hit seems slightly reduntant now

        let maybe_surface = maybe_precomp
            .as_ref()
            .map(|pc| pc.shade_hit(&self.light, is_shadowed));

        let reflected = self.reflected_colour(maybe_precomp, ref_lim - 1);

        // if in shadow should this just return black?
        maybe_surface
            .map(|surface| surface + reflected)
            .unwrap_or(Colour::black())
    }

    fn is_shadowed(&self, point: Tup) -> bool {
        let v = self.light.position.sub(point);
        let distance = v.length();
        let direction = v.norm();

        // cast ray between light source and ray intersection point
        let ray = Ray::new(point, direction);

        let maybe_intersect = ray.intersect_objects(&self.objects);
        let maybe_hit = maybe_intersect.hit();

        maybe_hit.map(|h| h.at < distance).unwrap_or(false)
    }

    fn reflected_colour(&self, comps: Option<PreComp>, ref_lim: u32) -> Colour {
        if ref_lim == 0 {
            return Colour::black();
        }
        if let Some(comps) = comps {
            if comps.object.material().reflectivity == 0.0 {
                Colour::black()
            } else {
                let reflect_ray = Ray::new(comps.over_point, comps.reflect_v);
                let colour = self.color_at(&reflect_ray, ref_lim);
                colour * comps.object.material().reflectivity
            }
        } else {
            Colour::black()
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let s1 = Sphere::builder()
            .with_transform(Matrix::ident())
            .with_material(Material::new(
                0.1,
                0.7,
                0.2,
                200.0,
                Colour::new(0.8, 1.0, 0.6),
                None,
                0.0,
            ))
            .build_trait();
        let s2 = Sphere::builder()
            .with_transform(Matrix::scaling(0.5, 0.5, 0.5))
            .build_trait();
        Self {
            objects: vec![s1, s2],
            light: PointLight::default(),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{
        colour::colour::Colour,
        geometry::vector::{point, vector},
        light::{self, light::PointLight},
        material::material::Material,
        matrix::matrix::Matrix,
        ray::ray::{Intersection, Ray},
        shapes::{plane::Plane, shape::TShape, sphere::Sphere},
        utils::test::ApproxEq,
        world,
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
        assert_eq!(sut[0].at, 4.0);
        assert_eq!(sut[1].at, 4.5);
        assert_eq!(sut[2].at, 5.5);
        assert_eq!(sut[3].at, 6.0);
    }
    #[test]
    fn shading_at_intersection_is_correct_from_outside() {
        let w = World::default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = &w.objects[0];
        let i = Intersection::new(4.0, shape.to_trait_ref());
        let comp = r.prep_comps(&i).unwrap();
        let c = comp.shade_hit(&w.light, false);
        c.approx_eq(Colour::new(0.38066, 0.47583, 0.2855));
    }
    #[test]
    fn shading_at_intersection_is_correct_from_inside() {
        let mut w = World::default();
        w.light = PointLight::new(point(0.0, 0.25, 0.0), Colour::white());
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let i = Intersection::new(0.5, shape.to_trait_ref());
        let comp = r.prep_comps(&i).unwrap();
        let c = comp.shade_hit(&w.light, false);
        c.approx_eq(Colour::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn precomp_will_cast_shadow() {
        let s1 = Sphere::builder().build_trait();
        let s2 = Sphere::builder()
            .with_transform(Matrix::translation(0.0, 0.0, 10.0))
            .build_trait();
        let s2_copy = Sphere::builder()
            .with_transform(Matrix::translation(0.0, 0.0, 10.0))
            .build_trait();

        let light = PointLight::new(point(0.0, 0.0, -10.0), Colour::white());

        let world = World::new(vec![s1, s2], light.clone());

        let ray = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let intersect = Intersection::new(4.0, s2_copy.to_trait_ref());
        let comps = ray.prep_comps(&intersect).unwrap();
        let shade_hit = comps.shade_hit(&light.clone(), world.is_shadowed(comps.point));
        shade_hit.approx_eq(Colour::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn precomp_hit_should_offset_the_point() {
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = Sphere::builder()
            .with_transform(Matrix::translation(0.0, 0.0, 1.0))
            .build_trait();
        let intersection = Intersection::new(5.0, shape.to_trait_ref());
        let comps = ray.prep_comps(&intersection).unwrap();
        assert!(comps.over_point.2 < (-0.00001) / 2.0);
        assert!(comps.point.2 > comps.over_point.2);
    }

    #[test]
    fn no_shadow_with_object_collinear_with_point_and_light() {
        let w = World::default();
        let p = point(0.0, 10.0, 0.0);
        let sut = w.is_shadowed(p);
        assert_eq!(sut, false)
    }
    #[test]
    fn shadow_with_object_between_point_and_light() {
        let w = World::default();
        let p = point(10.0, -10.0, 10.0);
        let sut = w.is_shadowed(p);
        assert_eq!(sut, true)
    }
    #[test]
    fn no_shadow_when_object_behind_the_light() {
        let w = World::default();
        let p = point(-20.0, 20.0, -20.0);
        let sut = w.is_shadowed(p);
        assert_eq!(sut, false)
    }
    #[test]
    fn no_shadow_when_object_behind_the_point() {
        let w = World::default();
        let p = point(-2.0, 2.0, -2.0);
        let sut = w.is_shadowed(p);
        assert_eq!(sut, false)
    }
    #[test]
    fn reflected_colour_for_non_reflective_material() {
        let s1 = Sphere::builder()
            .with_transform(Matrix::ident())
            .with_material(Material::new(
                0.1,
                0.7,
                0.2,
                200.0,
                Colour::new(0.8, 1.0, 0.6),
                None,
                0.0,
            ))
            .build_trait();
        let s2 = Sphere::builder()
            .with_material(Material::builder().with_ambient(1.0).build())
            .with_transform(Matrix::scaling(0.5, 0.5, 0.5))
            .build_trait();

        let world = World::new(vec![s1, s2], PointLight::default());
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));

        let i = Intersection::new(1.0, world.objects[1].to_trait_ref());
        let comps = r.prep_comps(&i);
        let colour = world.reflected_colour(comps, 5);
        assert_eq!(colour, Colour::black())
    }
    #[test]
    fn reflected_colour_for_reflective_material() {
        let s1 = Sphere::builder()
            .with_transform(Matrix::ident())
            .with_material(Material::new(
                0.1,
                0.7,
                0.2,
                200.0,
                Colour::new(0.8, 1.0, 0.6),
                None,
                0.0,
            ))
            .build_trait();
        let s2 = Sphere::builder()
            .with_transform(Matrix::scaling(0.5, 0.5, 0.5))
            .build_trait();
        let p1 = Plane::builder()
            .with_material(Material::builder().with_reflectivity(0.5).build())
            .with_transform(Matrix::translation(0.0, -1.0, 0.0))
            .build_trait();

        let world = World::new(vec![p1, s1, s2], PointLight::default());
        let r = Ray::new(
            point(0.0, 0.0, -3.0),
            vector(0.0, -(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0),
        );

        let i = Intersection::new(2.0_f64.sqrt(), world.objects[0].to_trait_ref());
        let comps = r.prep_comps(&i);
        let colour = world.reflected_colour(comps, 5);
        colour.approx_eq(Colour::new(0.19033, 0.23791, 0.14274))
    }

    #[test]
    fn reflected_colour_for_reflective_material_with_shade_hit() {
        let s1 = Sphere::builder()
            .with_transform(Matrix::ident())
            .with_material(Material::new(
                0.1,
                0.7,
                0.2,
                200.0,
                Colour::new(0.8, 1.0, 0.6),
                None,
                0.0,
            ))
            .build_trait();
        let s2 = Sphere::builder()
            .with_transform(Matrix::scaling(0.5, 0.5, 0.5))
            .build_trait();
        let p1 = Plane::builder()
            .with_material(Material::builder().with_reflectivity(0.5).build())
            .with_transform(Matrix::translation(0.0, -1.0, 0.0))
            .build_trait();

        let world = World::new(vec![p1, s1, s2], PointLight::default());
        let r = Ray::new(
            point(0.0, 0.0, -3.0),
            vector(0.0, -(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0),
        );

        let i = Intersection::new(2.0_f64.sqrt(), world.objects[0].to_trait_ref());
        let comps = r.prep_comps(&i).unwrap();
        let colour = world.color_at(&r, 5);
        colour.approx_eq(Colour::new(0.87675, 0.92434, 0.82918))
    }
    #[test]
    fn reflection_does_not_cause_stack_overflow() {
        let p1 = Plane::builder()
            .with_material(Material::builder().with_reflectivity(1.0).build())
            .with_transform(Matrix::translation(0.0, -1.0, 0.0))
            .build_trait();
        let p2 = Plane::builder()
            .with_material(Material::builder().with_reflectivity(1.0).build())
            .with_transform(Matrix::translation(0.0, 1.0, 0.0))
            .build_trait();

        let world = World::new(
            vec![p1, p2],
            PointLight::new(point(0.0, 0.0, 0.0), Colour::white()),
        );

        let ray = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0));
        let _ = world.color_at(&ray, 5);
    }
}
