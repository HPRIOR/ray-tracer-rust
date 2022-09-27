#![allow(dead_code, unused_variables, unreachable_patterns)]

use crate::colour::colour::Colour;
use crate::light::light::PointLight;
use crate::shapes::shape::TShape;
use crate::{
    geometry::vector::{point, Operations, Tup, Vector},
    matrix::matrix::Matrix,
    utils::math_ext::Square,
};

// ----------- Intersection ----------- //

/// Holds information about where a ray has intersected an object. It contains a reference to the
/// hit object
pub struct Intersection<'a> {
    pub at: f64,
    pub object: &'a Box<dyn TShape + 'a>,
}

impl<'a> Intersection<'a> {
    pub fn new(at: f64, object: &'a Box<dyn TShape + 'a>) -> Self {
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
    object: &'a Box<dyn TShape + 'a>,
    pub point: Tup,
    pub over_point: Tup,
    eye_v: Tup,
    norm_v: Tup,
    inside: bool,
}

impl<'a> PreComp<'a> {
    pub fn shade_hit(&self, light_source: &PointLight, is_shadow: bool) -> Colour {
        self.object.material().lighting(
            self.point,
            light_source,
            self.eye_v,
            self.norm_v,
            is_shadow,
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

    // The logic for intersects will have to change depending on the shape. The logic will
    // need to be delegated to the TShape trait: Tshape fn (&ray) -> Vec<Box<dyn TIntersection>>
    // not sure that the intersect needs to be a trait if they already reference the TShape trait
    pub fn intersect<'a>(&'a self, shape: &'a Box<dyn TShape + 'a>) -> Vec<Intersection<'a>> {
        if let Some(shape_transform) = shape.transform().inverse() {
            let new_ray = self.transform(&shape_transform);
            let sphere_to_ray = new_ray.origin.sub(point(0.0, 0.0, 0.0));

            let a = new_ray.direction.dot(new_ray.direction);
            let b = (new_ray.direction.dot(sphere_to_ray)) * 2.0;
            let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

            // if negative then ray misses - no intersection
            let discriminant = b.squared() - 4.0 * a * c;

            if discriminant < 0.0 {
                return vec![];
            }

            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            let i1 = Intersection::new(t1, shape);
            let i2 = Intersection::new(t2, shape);
            vec![i1, i2]
        } else {
            vec![]
        }
    }

    /// Returns a vector of intersections ordered from nearest to farthest
    pub fn intersect_objects<'a>(
        &'a self,
        shapes: &'a Vec<Box<dyn TShape>>,
    ) -> Vec<Intersection<'a>> {
        let mut result: Vec<Intersection<'a>> =
            shapes.iter().flat_map(|o| self.intersect(o)).collect();
        result.sort_by(|a, b| a.at.total_cmp(&b.at));
        result
    }

    pub fn prep_comps<'a>(&'a self, intersection: &Intersection<'a>) -> Option<PreComp> {
        let object = intersection.object;
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
        shapes::{shape::TShape, sphere::Sphere},
    };

    use super::{Hit, Intersection, Ray};

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

        let sphere: Box<dyn TShape> = Sphere::as_trait();

        let xs = ray.intersect(&sphere);
        assert_eq!(xs.len(), 2);

        assert_eq!(xs[0].at, 4.0);
        assert_eq!(xs[1].at, 6.0);
    }

    #[test]
    fn intersects_a_sphere_at_tangent() {
        let origin = point(0.0, 1.0, -5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere: Box<dyn TShape> = Sphere::as_trait();

        let xs = ray.intersect(&sphere);
        assert_eq!(xs.len(), 2);

        assert_eq!(xs[0].at, 5.0);
        assert_eq!(xs[1].at, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let origin = point(0.0, 2.0, -5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere: Box<dyn TShape> = Sphere::as_trait();

        let xs = ray.intersect(&sphere);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_can_originate_inside_sphere() {
        let origin = point(0.0, 0.0, 0.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere: Box<dyn TShape> = Sphere::as_trait();

        let xs = ray.intersect(&sphere);
        assert_eq!(xs.len(), 2);

        assert_eq!(xs[0].at, -1.0);
        assert_eq!(xs[1].at, 1.0);
    }

    #[test]
    fn sphere_behind_ray_has_negative_intersect() {
        let origin = point(0.0, 0.0, 5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere: Box<dyn TShape> = Sphere::as_trait();

        let xs = ray.intersect(&sphere);
        assert_eq!(xs.len(), 2);

        assert_eq!(xs[0].at, -6.0);
        assert_eq!(xs[1].at, -4.0);
    }

    #[test]
    fn intersect_set_the_object_on_intersection() {
        let origin = point(0.0, 0.0, -5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);
        let sphere: Box<dyn TShape> = Sphere::as_trait();
        let sut = ray.intersect(&sphere);
        assert_eq!(sut.len(), 2);

        let o1 = sut[0].object;
        let o2 = sut[1].object;

        let other_sphere: Box<dyn TShape> = Sphere::as_trait();

        assert!(std::ptr::eq(o1, o2));
        assert!(std::ptr::eq(o1, &sphere));
        assert!(std::ptr::eq(o2, &sphere));
        assert!(!std::ptr::eq(o1, &other_sphere));
        assert!(!std::ptr::eq(o2, &other_sphere));
    }

    #[test]
    fn correct_hit_when_all_intersections_have_positive_t() {
        let s: Box<dyn TShape> = Sphere::as_trait();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2];
        let sut = xs.hit().unwrap();
        assert!(std::ptr::eq(&xs[0], sut));
    }

    #[test]
    fn correct_hit_when_all_intersections_some_intersections_have_negative_t() {
        let s: Box<dyn TShape> = Sphere::as_trait();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = vec![i1, i2];
        let sut = xs.hit().unwrap();
        assert!(std::ptr::eq(&xs[1], sut));
    }

    #[test]
    fn correct_hit_when_all_intersections_all_intersections_have_negative_t() {
        let s: Box<dyn TShape> = Sphere::as_trait();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = vec![i1, i2];
        let sut = xs.hit();
        assert!(sut.is_none());
    }

    #[test]
    fn hit_is_lowest_non_negative_intersection() {
        let s: Box<dyn TShape> = Sphere::as_trait();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
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
        let s: Box<dyn TShape> = Sphere::as_trait_with_transform(m);
        let xs = r1.intersect(&s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].at, 3.0);
        assert_eq!(xs[1].at, 7.0);
    }
    #[test]
    fn intersecting_translated_sphere_with_a_ray() {
        let r1 = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let m = Matrix::translation(5.0, 0.0, 0.0);
        let s: Box<dyn TShape> = Sphere::as_trait_with_transform(m);
        let xs = r1.intersect(&s);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn precomputing_intersection_state() {
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape: Box<dyn TShape> = Sphere::as_trait();
        let i = Intersection {
            at: 4.0,
            object: &shape,
        };
        let comps = ray.prep_comps(&i).unwrap();
        let comps_obj = comps.object;
        let intersect_obj = i.object;
        // intersect and precom reference the same obj
        assert!(std::ptr::eq(comps_obj, intersect_obj));

        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eye_v, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.norm_v, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn inside_is_false_when_intersection_occurs_on_the_outsied() {
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = Sphere::as_trait();
        let i = Intersection::new(4.0, &shape);
        let comps = ray.prep_comps(&i).unwrap();
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn intersection_when_intersection_occurs_on_the_inside() {
        let ray = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = Sphere::as_trait();
        let i = Intersection::new(1.0, &shape);
        let comps = ray.prep_comps(&i).unwrap();

        assert_eq!(comps.point, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eye_v, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
        assert_eq!(comps.norm_v, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn can_get_ordered_intersects_with_multiple_objects() {
        let s1 = Sphere::as_trait_with_attr(
            Matrix::ident(),
            Material::new(0.1, 0.7, 0.2, 200.0, Colour::new(0.8, 1.0, 0.6)),
        );
        let s2 = Sphere::as_trait_with_transform(Matrix::scaling(0.5, 0.5, 0.5));
        let objects = vec![s1, s2];
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));

        let sut = ray.intersect_objects(&objects);
        assert_eq!(sut.len(), 4);
        assert_eq!(sut[0].at, 4.0);
        assert_eq!(sut[1].at, 4.5);
        assert_eq!(sut[2].at, 5.5);
        assert_eq!(sut[3].at, 6.0);
    }
}
