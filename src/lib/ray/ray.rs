#![allow(dead_code, unused_variables, unreachable_patterns)]

use crate::shapes::shape::Shape;
use crate::{
    geometry::vector::{point, Operations, Tup, Vector},
    matrix::matrix::Matrix,
    utils::math_ext::Square,
};

// ----------- Intersection ----------- //

pub struct Intersection<'a> {
    // type will have to be more constrained if intersection implements any members which make use
    // of the object
    pub at: f64,
    pub object: &'a Box<dyn Shape + 'a>,
}

impl<'a> Intersection<'a> {
    pub fn new(at: f64, object: &'a Box<dyn Shape + 'a>) -> Self {
        Self { at, object }
    }
    pub fn as_trait(at: f64, object: &'a Box<dyn Shape + 'a>) -> Box<dyn TIntersection<'a> + 'a> {
        Box::new(Self { at, object })
    }
}

pub trait TIntersection<'a> {
    fn at(&self) -> f64;
    fn object(&self) -> &'a Box<dyn Shape + 'a>;
}

impl<'a> TIntersection<'a> for Intersection<'a> {
    fn at(&self) -> f64 {
        self.at
    }

    fn object(&self) -> &'a Box<dyn Shape + 'a> {
        self.object
    }
}

pub trait Hit {
    type Output;

    fn hit(&self) -> Option<&Self::Output>;
}

impl<'a> Hit for Vec<Box<dyn TIntersection<'a> + 'a>> {
    type Output = Box<dyn TIntersection<'a> + 'a>;

    fn hit(&self) -> Option<&Self::Output> {
        if self.len() == 0 {
            return None;
        };

        let mut positive_intersections: Vec<&Self::Output> =
            self.into_iter().filter(|i| i.at() > 0.0).collect();
        if positive_intersections.len() == 0 {
            return None;
        }
        positive_intersections.sort_by(|a, b| a.at().total_cmp(&b.at()));
        Some(&positive_intersections[0])
    }
}

// ----------- PreComp ----------- //
struct PreComp<'a> {
    object: &'a Box<dyn Shape + 'a>,
    point: Tup,
    eye_v: Tup,
    norm_v: Tup,
}

// ----------- Ray ----------- //
#[derive(Debug)]
pub struct Ray {
    origin: Tup,
    pub direction: Tup,
}

impl Ray {
    pub fn new(origin: Tup, direction: Tup) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, t: f64) -> Tup {
        self.direction.mul(t).add(self.origin)
    }

    // this will need to change for runtime polymorphism. Should return Box<dyn Hit>
    pub fn intersect<'a>(
        &'a self,
        shape: &'a Box<dyn Shape + 'a>,
    ) -> Vec<Box<dyn TIntersection<'a> + 'a>> {
        if let Some(sphere_transform) = shape.transform().inverse() {
            let new_ray = self.transform(&sphere_transform);
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

            let i1: Box<dyn TIntersection<'a>> = Intersection::as_trait(t1, shape);
            let i2: Box<dyn TIntersection<'a>> = Intersection::as_trait(t2, shape);
            vec![i1, i2]
        } else {
            vec![]
        }
    }

    fn prepare_computations<'a>(
        &'a self,
        intersection: &Box<dyn TIntersection<'a> + 'a>,
    ) -> Option<PreComp> {
        let object = intersection.object();
        let p = self.position(intersection.at());
        let eye_v = self.direction.neg();
        let norm_v_opt = object.normal_at(p);
        norm_v_opt.map(|norm_v| PreComp {
            object,
            point: p,
            eye_v,
            norm_v,
        })
    }

    fn transform(&self, transform: &Matrix) -> Self {
        Self {
            origin: transform.mul_tup(self.origin),
            direction: transform.mul_tup(self.direction),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        geometry::vector::{point, vector},
        matrix::matrix::Matrix,
        ray::ray::TIntersection,
        shapes::{shape::Shape, sphere::Sphere},
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

        let sphere: Box<dyn Shape> = Sphere::as_trait();

        let xs = ray.intersect(&sphere);
        assert_eq!(xs.len(), 2);

        assert_eq!(xs[0].at(), 4.0);
        assert_eq!(xs[1].at(), 6.0);
    }

    #[test]
    fn intersects_a_sphere_at_tangent() {
        let origin = point(0.0, 1.0, -5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere: Box<dyn Shape> = Sphere::as_trait();

        let xs = ray.intersect(&sphere);
        assert_eq!(xs.len(), 2);

        assert_eq!(xs[0].at(), 5.0);
        assert_eq!(xs[1].at(), 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let origin = point(0.0, 2.0, -5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere: Box<dyn Shape> = Sphere::as_trait();

        let xs = ray.intersect(&sphere);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_can_originate_inside_sphere() {
        let origin = point(0.0, 0.0, 0.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere: Box<dyn Shape> = Sphere::as_trait();

        let xs = ray.intersect(&sphere);
        assert_eq!(xs.len(), 2);

        assert_eq!(xs[0].at(), -1.0);
        assert_eq!(xs[1].at(), 1.0);
    }

    #[test]
    fn sphere_behind_ray_has_negative_intersect() {
        let origin = point(0.0, 0.0, 5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere: Box<dyn Shape> = Sphere::as_trait();

        let xs = ray.intersect(&sphere);
        assert_eq!(xs.len(), 2);

        assert_eq!(xs[0].at(), -6.0);
        assert_eq!(xs[1].at(), -4.0);
    }

    #[test]
    fn intersect_set_the_object_on_intersection() {
        let origin = point(0.0, 0.0, -5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);
        let sphere: Box<dyn Shape> = Sphere::as_trait();
        let sut = ray.intersect(&sphere);
        assert_eq!(sut.len(), 2);

        let o1 = sut[0].object();
        let o2 = sut[1].object();

        let other_sphere: Box<dyn Shape> = Sphere::as_trait();

        assert!(std::ptr::eq(o1, o2));
        assert!(std::ptr::eq(o1, &sphere));
        assert!(std::ptr::eq(o2, &sphere));
        assert!(!std::ptr::eq(o1, &other_sphere));
        assert!(!std::ptr::eq(o2, &other_sphere));
    }

    #[test]
    fn correct_hit_when_all_intersections_have_positive_t() {
        let s: Box<dyn Shape> = Sphere::as_trait();
        let i1: Box<dyn TIntersection> = Intersection::as_trait(1.0, &s);
        let i2: Box<dyn TIntersection> = Intersection::as_trait(2.0, &s);
        let xs = vec![i1, i2];
        let sut = xs.hit().unwrap();
        assert!(std::ptr::eq(&xs[0], sut));
    }

    #[test]
    fn correct_hit_when_all_intersections_some_intersections_have_negative_t() {
        let s: Box<dyn Shape> = Sphere::as_trait();
        let i1: Box<dyn TIntersection> = Intersection::as_trait(-1.0, &s);
        let i2: Box<dyn TIntersection> = Intersection::as_trait(1.0, &s);
        let xs = vec![i1, i2];
        let sut = xs.hit().unwrap();
        assert!(std::ptr::eq(&xs[1], sut));
    }

    #[test]
    fn correct_hit_when_all_intersections_all_intersections_have_negative_t() {
        let s: Box<dyn Shape> = Sphere::as_trait();
        let i1: Box<dyn TIntersection> = Intersection::as_trait(-1.0, &s);
        let i2: Box<dyn TIntersection> = Intersection::as_trait(-1.0, &s);
        let xs = vec![i1, i2];
        let sut = xs.hit();
        assert!(sut.is_none());
    }

    #[test]
    fn hit_is_lowest_non_negative_intersection() {
        let s: Box<dyn Shape> = Sphere::as_trait();
        let i1: Box<dyn TIntersection> = Intersection::as_trait(5.0, &s);
        let i2: Box<dyn TIntersection> = Intersection::as_trait(7.0, &s);
        let i3: Box<dyn TIntersection> = Intersection::as_trait(-3.0, &s);
        let i4: Box<dyn TIntersection> = Intersection::as_trait(2.0, &s);
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
        let s: Box<dyn Shape> = Sphere::as_trait_with_transform(m);
        let xs = r1.intersect(&s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].at(), 3.0);
        assert_eq!(xs[1].at(), 7.0);
    }
    #[test]
    fn intersecting_translated_sphere_with_a_ray() {
        let r1 = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let m = Matrix::translation(5.0, 0.0, 0.0);
        let s: Box<dyn Shape> = Sphere::as_trait_with_transform(m);
        let xs = r1.intersect(&s);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn precomputing_intersection_state() {
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape: Box<dyn Shape> = Sphere::as_trait();
        let i: Box<dyn TIntersection> = Box::new(Intersection {
            at: 4.0,
            object: &shape,
        });
        let comps = ray.prepare_computations(&i).unwrap();
        let comps_obj = comps.object;
        let intersect_obj = i.object();
        // intersect and precom reference the same obj
        assert!(std::ptr::eq(comps_obj, intersect_obj));

        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eye_v, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.norm_v, vector(0.0, 0.0, -1.0));
    }
}
