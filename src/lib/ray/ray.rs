#![allow(dead_code, unused_variables, unreachable_patterns)]

use crate::{
    geometry::vector::{point, Operations, Tup, Vector},
    matrix::matrix::Matrix,
    shapes::sphere::Sphere,
    utils::math_ext::Square,
};

#[derive(Debug)]
pub enum Object<'a> {
    Sphere(&'a Sphere),
}

#[derive(Debug)]
pub struct Intersection<'a> {
    pub at: f64,
    pub object: Object<'a>,
}

impl<'a> Intersection<'a> {
    pub fn new(at: f64, object: Object<'a>) -> Self {
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

    /// Values are where the sphere is intersected on the ray from the origin or None if no
    /// intersection
    pub fn intersect<'a>(&'a self, sphere: &'a Sphere) -> Vec<Intersection> {
        if let Some(sphere_transform) = sphere.transform.inverse() {
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

            let i1 = Intersection::new(t1, Object::Sphere(sphere));
            let i2 = Intersection::new(t2, Object::Sphere(sphere));
            vec![i1, i2]
        } else {
            vec![]
        }
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
        shapes::sphere::Sphere,
    };

    use super::{Hit, Intersection, Object, Ray};

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

        let sphere = Sphere::new();

        let xs = ray.intersect(&sphere);
        assert!(xs.len() == 2);

        assert_eq!(xs[0].at, 4.0);
        assert_eq!(xs[1].at, 6.0);
    }

    #[test]
    fn intersects_a_sphere_at_tengent() {
        let origin = point(0.0, 1.0, -5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere = Sphere::new();

        let xs = ray.intersect(&sphere);
        assert!(xs.len() == 2);

        assert_eq!(xs[0].at, 5.0);
        assert_eq!(xs[1].at, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let origin = point(0.0, 2.0, -5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere = Sphere::new();

        let xs = ray.intersect(&sphere);
        assert!(xs.len() == 0);
    }

    #[test]
    fn ray_can_originate_inside_sphere() {
        let origin = point(0.0, 0.0, 0.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere = Sphere::new();

        let xs = ray.intersect(&sphere);
        assert!(xs.len() == 2);

        assert_eq!(xs[0].at, -1.0);
        assert_eq!(xs[1].at, 1.0);
    }

    #[test]
    fn sphere_behind_ray_has_negative_intersect() {
        let origin = point(0.0, 0.0, 5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere = Sphere::new();

        let xs = ray.intersect(&sphere);
        assert!(xs.len() == 2);

        assert_eq!(xs[0].at, -6.0);
        assert_eq!(xs[1].at, -4.0);
    }

    #[test]
    fn intersect_set_the_object_on_intersection() {
        let origin = point(0.0, 0.0, -5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);
        let sphere = Sphere::new();
        let sut = ray.intersect(&sphere);
        assert!(sut.len() == 2);

        let o1 = match sut[0].object {
            Object::Sphere(s) => s,
            _ => panic!(),
        };
        let o2 = match sut[1].object {
            Object::Sphere(s) => s,
            _ => panic!(),
        };

        let other_sphere = Sphere::new();

        assert!(std::ptr::eq(o1, o2));
        assert!(std::ptr::eq(o1, &sphere));
        assert!(std::ptr::eq(o2, &sphere));
        assert!(!std::ptr::eq(o1, &other_sphere));
        assert!(!std::ptr::eq(o2, &other_sphere));
    }

    #[test]
    fn correct_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, Object::Sphere(&s));
        let i2 = Intersection::new(2.0, Object::Sphere(&s));
        let xs = vec![i1, i2];
        let sut = xs.hit().unwrap();
        assert!(std::ptr::eq(&xs[0], sut));
    }

    #[test]
    fn correct_hit_when_all_intersections_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, Object::Sphere(&s));
        let i2 = Intersection::new(1.0, Object::Sphere(&s));
        let xs = vec![i1, i2];
        let sut = xs.hit().unwrap();
        assert!(std::ptr::eq(&xs[1], sut));
    }

    #[test]
    fn correct_hit_when_all_intersections_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, Object::Sphere(&s));
        let i2 = Intersection::new(-1.0, Object::Sphere(&s));
        let xs = vec![i1, i2];
        let sut = xs.hit();
        assert!(sut.is_none());
    }

    #[test]
    fn hit_is_lowest_non_negative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, Object::Sphere(&s));
        let i2 = Intersection::new(7.0, Object::Sphere(&s));
        let i3 = Intersection::new(-3.0, Object::Sphere(&s));
        let i4 = Intersection::new(2.0, Object::Sphere(&s));
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
        let s = Sphere::with_transform(m);
        let xs = r1.intersect(&s);

        assert!(xs.len() == 2);
        assert_eq!(xs[0].at, 3.0);
        assert_eq!(xs[1].at, 7.0);
    }
    #[test]
    fn intersecting_translated_sphere_with_a_ray() {
        let r1 = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let m = Matrix::translation(5.0, 0.0, 0.0);
        let s = Sphere::with_transform(m);
        let xs = r1.intersect(&s);

        assert!(xs.len() == 0);
    }
}
