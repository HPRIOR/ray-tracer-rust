#![allow(dead_code, unused_variables, unreachable_patterns)]

use crate::colour::colour::Colour;
use crate::light::light::PointLight;
use crate::shapes::shape::TShape;
use crate::{
    geometry::vector::{Operations, Tup, Vector},
    matrix::matrix::Matrix,
};

// ----------- Intersection ----------- //

/// Holds information about where a ray has intersected an object. It contains a reference to the
/// hit object
#[derive(Debug)]
pub struct Intersection<'a> {
    /// Where on an object a ray intersects
    pub at: f64,
    pub object: Box<&'a (dyn TShape + 'a)>,
}

impl<'a> Intersection<'a> {
    pub fn new(at: f64, object: Box<&'a (dyn TShape + 'a)>) -> Self {
        Self { at, object }
    }
}

pub trait Hit {
    type Output;

    fn hit(&self) -> Option<&Self::Output>;
}

impl<'a> Hit for Vec<Intersection<'a>> {
    type Output = Intersection<'a>;

    fn hit(&self) -> Option<&Self::Output> {
        if self.len() == 0 {
            return None;
        };

        let mut positive_intersections: Vec<&Self::Output> =
            self.into_iter().filter(|i| i.at > 0.0).collect();
        if positive_intersections.len() == 0 {
            return None;
        }
        positive_intersections.sort_by(|a, b| a.at.total_cmp(&b.at));
        Some(&positive_intersections[0])
    }
}

// ----------- PreComp ----------- //
pub struct PreComp<'a> {
    pub object: Box<&'a (dyn TShape + 'a)>,
    pub point: Tup,
    pub over_point: Tup,
    eye_v: Tup,
    norm_v: Tup,
    inside: bool,
    pub reflect_v: Tup,
    n1: f64,
    n2: f64,
}

impl<'a> PreComp<'a> {
    pub fn shade_hit(&self, light_source: &PointLight, is_shadow: bool) -> Colour {
        self.object.material().lighting(
            self.point,
            light_source,
            self.eye_v,
            self.norm_v,
            is_shadow,
            self.object.to_trait_ref(),
        )
    }
}

// ----------- Ray ----------- //
#[derive(Debug)]
pub struct Ray {
    pub origin: Tup,
    pub direction: Tup,
}

impl Ray {
    pub fn new(origin: Tup, direction: Tup) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, t: f64) -> Tup {
        self.direction.mul(t).add(self.origin)
    }

    /// Returns a vector of intersections ordered from nearest to farthest
    /// The actual intersection of the ray is delegated to the TShape trait so that any group of
    /// shapes can be intersected
    pub fn intersect_objects<'a>(&self, shapes: &'a Vec<Box<dyn TShape>>) -> Vec<Intersection<'a>> {
        let mut result: Vec<Intersection<'a>> =
            shapes.into_iter().flat_map(|o| o.intersect(self)).collect();

        result.sort_by(|a, b| a.at.total_cmp(&b.at));
        result
    }

    pub fn prep_comp<'a>(
        &'a self,
        intersection: &Intersection<'a>,
        xs: &Vec<&Intersection<'a>>,
    ) -> Option<PreComp> {
        let object = intersection.object.to_trait_ref();
        let p = self.position(intersection.at);
        let eye_v = self.direction.neg();
        let maybe_norm_v = object.normal_at(p);

        maybe_norm_v.map(|norm_v| {
            // if hit occurs inside the shape then we must invert the normal
            let is_inside = norm_v.dot(eye_v) < 0.0;
            let norm_v_result = if is_inside { norm_v.neg() } else { norm_v };

            PreComp {
                object,
                point: p,
                over_point: p.add(norm_v_result.mul(0.00001)),
                eye_v,
                norm_v: norm_v_result,
                inside: is_inside,
                reflect_v: self.direction.reflect(norm_v.neg()),
                n1: 1.1,
                n2: 1.2,
            }
        })
    }

    pub fn transform(&self, transform: &Matrix) -> Self {
        Self {
            origin: transform.mul_tup(self.origin),
            direction: transform.mul_tup(self.direction),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        colour::colour::Colour,
        geometry::vector::{point, vector},
        material::material::Material,
        matrix::matrix::Matrix,
        shapes::{
            plane::Plane,
            shape::{TShape, TShapeBuilder},
            sphere::Sphere,
        },
    };

    use super::{Hit, Intersection, Ray};

    fn glass_sphere(transform: Matrix, ref_index: f64) -> Sphere {
        Sphere::builder()
            .with_transform(transform)
            .with_material(
                Material::builder()
                    .with_transparency(1.0)
                    .with_refractive_index(ref_index)
                    .build(),
            )
            .build()
    }

    #[test]
    fn ray_can_be_created_with_origin_and_direction() {
        let origin = point(1.0, 2.0, 3.0);
        let direction = vector(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn compute_a_point_from_distance() {
        let origin = point(2.0, 3.0, 4.0);
        let direction = vector(1.0, 0.0, 0.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray.position(0.0), point(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), point(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), point(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), point(4.5, 3.0, 4.0));
    }

    #[test]
    fn intersects_a_sphere_at_two_points() {
        let origin = point(0.0, 0.0, -5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere: Box<dyn TShape> = Sphere::builder().build_trait();

        let xs = sphere.intersect(&ray);
        assert_eq!(xs.len(), 2);

        assert_eq!(xs[0].at, 4.0);
        assert_eq!(xs[1].at, 6.0);
    }

    #[test]
    fn intersects_a_sphere_at_tangent() {
        let origin = point(0.0, 1.0, -5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere: Box<dyn TShape> = Sphere::builder().build_trait();

        let xs = sphere.intersect(&ray);
        assert_eq!(xs.len(), 2);

        assert_eq!(xs[0].at, 5.0);
        assert_eq!(xs[1].at, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let origin = point(0.0, 2.0, -5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere: Box<dyn TShape> = Sphere::builder().build_trait();

        let xs = sphere.intersect(&ray);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_can_originate_inside_sphere() {
        let origin = point(0.0, 0.0, 0.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere: Box<dyn TShape> = Sphere::builder().build_trait();

        let xs = sphere.intersect(&ray);
        assert_eq!(xs.len(), 2);

        assert_eq!(xs[0].at, -1.0);
        assert_eq!(xs[1].at, 1.0);
    }

    #[test]
    fn sphere_behind_ray_has_negative_intersect() {
        let origin = point(0.0, 0.0, 5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere: Box<dyn TShape> = Sphere::builder().build_trait();

        let xs = sphere.intersect(&ray);
        assert_eq!(xs.len(), 2);

        assert_eq!(xs[0].at, -6.0);
        assert_eq!(xs[1].at, -4.0);
    }

    #[test]
    fn intersect_set_the_object_on_intersection() {
        let origin = point(0.0, 0.0, -5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);
        let sphere: Box<dyn TShape> = Sphere::builder().build_trait();
        let sut = sphere.intersect(&ray);
        assert_eq!(sut.len(), 2);

        let o1 = &sut[0].object;
        let o2 = &sut[1].object;

        let other_sphere: Box<dyn TShape> = Sphere::builder().build_trait();

        assert!(std::ptr::eq(*o1.as_ref(), *o2.as_ref()));
        assert!(std::ptr::eq(*o1.as_ref(), sphere.as_ref()));
        assert!(std::ptr::eq(*o2.as_ref(), sphere.as_ref()));
        assert!(!std::ptr::eq(*o1.as_ref(), other_sphere.as_ref()));
        assert!(!std::ptr::eq(*o2.as_ref(), other_sphere.as_ref()));
    }

    #[test]
    fn correct_hit_when_all_intersections_have_positive_t() {
        let s: Box<dyn TShape> = Sphere::builder().build_trait();
        let i1 = Intersection::new(1.0, s.to_trait_ref());
        let i2 = Intersection::new(2.0, s.to_trait_ref());
        let xs = vec![i1, i2];
        let sut = xs.hit().unwrap();
        assert!(std::ptr::eq(&xs[0], sut));
    }

    #[test]
    fn correct_hit_when_all_intersections_some_intersections_have_negative_t() {
        let s: Box<dyn TShape> = Sphere::builder().build_trait();
        let i1 = Intersection::new(-1.0, s.to_trait_ref());
        let i2 = Intersection::new(1.0, s.to_trait_ref());
        let xs = vec![i1, i2];
        let sut = xs.hit().unwrap();
        assert!(std::ptr::eq(&xs[1], sut));
    }

    #[test]
    fn correct_hit_when_all_intersections_all_intersections_have_negative_t() {
        let s: Box<dyn TShape> = Sphere::builder().build_trait();
        let i1 = Intersection::new(-1.0, s.to_trait_ref());
        let i2 = Intersection::new(-1.0, s.to_trait_ref());
        let xs = vec![i1, i2];
        let sut = xs.hit();
        assert!(sut.is_none());
    }

    #[test]
    fn hit_is_lowest_non_negative_intersection() {
        let s: Box<dyn TShape> = Sphere::builder().build_trait();
        let i1 = Intersection::new(5.0, s.to_trait_ref());
        let i2 = Intersection::new(7.0, s.to_trait_ref());
        let i3 = Intersection::new(-3.0, s.to_trait_ref());
        let i4 = Intersection::new(2.0, s.to_trait_ref());
        let xs = vec![i1, i2, i3, i4];
        let sut = xs.hit().unwrap();
        assert!(std::ptr::eq(&xs[3], sut));
    }

    #[test]
    fn ray_can_be_translated() {
        let r1 = Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = Matrix::translation(3.0, 4.0, 5.0);
        let r2 = r1.transform(&m);
        assert_eq!(r2.origin, point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn ray_can_be_scaled() {
        let r1 = Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = Matrix::scaling(2.0, 3.0, 4.0);
        let r2 = r1.transform(&m);
        assert_eq!(r2.origin, point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, vector(0.0, 3.0, 0.0));
    }
    #[test]
    fn intersecting_scaled_sphere_with_a_ray() {
        let r1 = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let m = Matrix::scaling(2.0, 2.0, 2.0);
        let s: Box<dyn TShape> = Sphere::builder().with_transform(m).build_trait();
        let xs = s.intersect(&r1);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].at, 3.0);
        assert_eq!(xs[1].at, 7.0);
    }
    #[test]
    fn intersecting_translated_sphere_with_a_ray() {
        let r1 = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let m = Matrix::translation(5.0, 0.0, 0.0);
        let s: Box<dyn TShape> = Sphere::builder().with_transform(m).build_trait();
        let xs = s.intersect(&r1);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn precomputing_intersection_state() {
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape: Box<dyn TShape> = Sphere::builder().build_trait();
        let i = Intersection {
            at: 4.0,
            object: shape.to_trait_ref(),
        };
        let comps = ray.prep_comp(&i, &vec![&i]).unwrap();
        let comps_obj = comps.object;
        let intersect_obj = i.object;
        // intersect and precom reference the same obj
        assert!(std::ptr::eq(*comps_obj.as_ref(), *intersect_obj.as_ref()));

        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eye_v, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.norm_v, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn inside_is_false_when_intersection_occurs_on_the_outsied() {
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = Sphere::builder().build_trait();
        let i = Intersection::new(4.0, shape.to_trait_ref());
        let comps = ray.prep_comp(&i, &vec![&i]).unwrap();
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn intersection_when_intersection_occurs_on_the_inside() {
        let ray = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = Sphere::builder().build_trait();
        let i = Intersection::new(1.0, shape.to_trait_ref());
        let comps = ray.prep_comp(&i, &vec![&i]).unwrap();

        assert_eq!(comps.point, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eye_v, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
        assert_eq!(comps.norm_v, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn can_get_ordered_intersects_with_multiple_objects() {
        let s1 = Sphere::builder()
            .with_transform(Matrix::ident())
            .with_material(
                Material::builder()
                    .with_diffuse(0.7)
                    .with_specular(0.2)
                    .with_colour(Colour::new(0.8, 1.0, 0.6))
                    .build(),
            )
            .build_trait();
        let s2 = Sphere::builder()
            .with_transform(Matrix::scaling(0.5, 0.5, 0.5))
            .build_trait();
        let objects = vec![s1, s2];
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));

        let sut = ray.intersect_objects(&objects);
        assert_eq!(sut.len(), 4);
        assert_eq!(sut[0].at, 4.0);
        assert_eq!(sut[1].at, 4.5);
        assert_eq!(sut[2].at, 5.5);
        assert_eq!(sut[3].at, 6.0);
    }

    #[test]
    fn precomputing_the_reflective_vector() {
        let shape = Plane::builder().build_trait();
        let ray = Ray::new(
            point(0.0, 1.0, -1.0),
            vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), shape.to_trait_ref());
        let comps = ray.prep_comp(&i, &vec![&i]).unwrap();
        assert_eq!(
            comps.reflect_v,
            vector(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn finding_n1_and_n1_at_various_intersections() {
        let a = glass_sphere(Matrix::scaling(2.0, 2.0, 2.0), 1.5);
        let b = glass_sphere(Matrix::translation(0.0, 0.0, -0.25), 2.0);
        let c = glass_sphere(Matrix::translation(0.0, 0.0, 0.25), 2.5);

        let ray = Ray::new(point(0.0, 0.0, -4.0), vector(0.0, 0.0, 1.0));
        let intersections: Vec<Intersection> = vec![
            Intersection::new(2.0, a.to_trait_ref()),
            Intersection::new(2.75, b.to_trait_ref()),
            Intersection::new(3.25, c.to_trait_ref()),
            Intersection::new(4.75, b.to_trait_ref()),
            Intersection::new(5.25, c.to_trait_ref()),
            Intersection::new(6.0, a.to_trait_ref()),
        ];

        let i_ref: Vec<&Intersection> = intersections.iter().collect();
        let xs = intersections
            .iter()
            .map(|i| ray.prep_comp(i, &i_ref))
            .filter_map(|x| x);

        let expected = vec![
            (1.0, 1.5),
            (1.5, 2.0),
            (2.0, 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0),
        ];

        xs.zip(expected).for_each(|val| {
            assert_eq!(val.0.n1, val.1 .0);
            assert_eq!(val.0.n2, val.1 .1)
        });
    }
}
