#![allow(dead_code, unused_variables, unreachable_patterns)]

use crate::{
    geometry::vector::{point, Operations, Tup, Vector},
    shapes::sphere::Sphere,
    utils::math_ext::Square,
};

use super::intersection::{Intersection, Object};

trait Hit {
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

/// Implemented for optional type for a nicer interface with ray.intersect() method which returns
/// an option - None if no intersection occurs
impl<'a> Hit for Option<Vec<Intersection<'a>>> {
    type Output = Intersection<'a>;

    fn hit(&self) -> Option<&Self::Output> {
        match self {
            Some(i) => {
                let mut unsorted: Vec<&Intersection> =
                    i.into_iter().filter(|i| i.at > 0.0).collect();
                if unsorted.len() == 0 {
                    return None;
                }
                unsorted.sort_by(|a, b| a.at.total_cmp(&b.at));
                Some(unsorted[0])
            }

            None => None,
        }
    }
}

pub struct Ray {
    origin: Tup,
    direction: Tup,
}

impl Ray {
    pub fn new(origin: Tup, direction: Tup) -> Self {
        Self { origin, direction }
    }

    fn position(&self, t: f64) -> Tup {
        self.direction.mul(t).add(self.origin)
    }

    /// Values are where the sphere is intersected on the ray from the origin or None if no
    /// intersection
    fn intersect<'a>(&'a self, sphere: &'a Sphere) -> Option<Vec<Intersection>> {
        // vector from the sphere's center, to the ray origin remember: the sphere is centered at
        // the world origin. We are also assuming a size of 1 for the sphere
        let sphere_to_ray = self.origin.sub(point(0.0, 0.0, 0.0));

        let a = self.direction.dot(self.direction);
        let b = (self.direction.dot(sphere_to_ray)) * 2.0;
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        // if negative then ray misses - no intersection
        let discriminant = b.squared() - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        let i1 = Intersection::new(t1, Object::Sphere(sphere));
        let i2 = Intersection::new(t2, Object::Sphere(sphere));
        Some(vec![i1, i2])
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        geometry::vector::{point, vector},
        ray::intersection::{Intersection, Object},
        shapes::sphere::Sphere,
    };

    use super::{Hit, Ray};

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
        assert!(xs.is_some());

        let sut = xs.unwrap();
        assert_eq!(sut[0].at, 4.0);
        assert_eq!(sut[1].at, 6.0);
    }

    #[test]
    fn intersects_a_sphere_at_tengent() {
        let origin = point(0.0, 1.0, -5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere = Sphere::new();

        let xs = ray.intersect(&sphere);
        assert!(xs.is_some());

        let sut = xs.unwrap();
        assert_eq!(sut[0].at, 5.0);
        assert_eq!(sut[1].at, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let origin = point(0.0, 2.0, -5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere = Sphere::new();

        let xs = ray.intersect(&sphere);
        assert!(xs.is_none());
    }

    #[test]
    fn ray_can_originate_inside_sphere() {
        let origin = point(0.0, 0.0, 0.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere = Sphere::new();

        let xs = ray.intersect(&sphere);
        assert!(xs.is_some());

        let sut = xs.unwrap();
        assert_eq!(sut[0].at, -1.0);
        assert_eq!(sut[1].at, 1.0);
    }

    #[test]
    fn sphere_behind_ray_has_negative_intersect() {
        let origin = point(0.0, 0.0, 5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere = Sphere::new();

        let xs = ray.intersect(&sphere);
        assert!(xs.is_some());

        let sut = xs.unwrap();
        assert_eq!(sut[0].at, -6.0);
        assert_eq!(sut[1].at, -4.0);
    }

    #[test]
    fn intersect_set_the_object_on_intersection() {
        let origin = point(0.0, 0.0, -5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);
        let sphere = Sphere::new();
        let sut = ray.intersect(&sphere);
        assert!(sut.is_some());

        let intersect = sut.unwrap();
        let o1 = match intersect[0].object {
            Object::Sphere(s) => s,
            _ => panic!(),
        };
        let o2 = match intersect[1].object {
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
    fn correct_hit_when_all_intersections_have_positive_t_opt() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, Object::Sphere(&s));
        let i2 = Intersection::new(2.0, Object::Sphere(&s));
        let xs = Some(vec![i1, i2]);
        let sut = xs.hit().unwrap();
        assert!(std::ptr::eq(&xs.as_ref().unwrap()[0], sut));
    }

    #[test]
    fn correct_hit_when_all_intersections_some_intersections_have_negative_t_opt() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, Object::Sphere(&s));
        let i2 = Intersection::new(1.0, Object::Sphere(&s));
        let xs = Some(vec![i1, i2]);
        let sut = xs.hit().unwrap();
        assert!(std::ptr::eq(&xs.as_ref().unwrap()[1], sut));
    }

    #[test]
    fn correct_hit_when_all_intersections_all_intersections_have_negative_t_opt() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, Object::Sphere(&s));
        let i2 = Intersection::new(-1.0, Object::Sphere(&s));
        let xs = Some(vec![i1, i2]);
        let sut = xs.hit();
        assert!(sut.is_none());
    }

    #[test]
    fn hit_is_lowest_non_negative_intersection_opt() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, Object::Sphere(&s));
        let i2 = Intersection::new(7.0, Object::Sphere(&s));
        let i3 = Intersection::new(-3.0, Object::Sphere(&s));
        let i4 = Intersection::new(2.0, Object::Sphere(&s));
        let xs = Some(vec![i1, i2, i3, i4]);
        let sut = xs.hit().unwrap();
        assert!(std::ptr::eq(&xs.as_ref().unwrap()[3], sut));
    }

    #[test]
    fn optional_hit_api_works() {
        let origin = point(0.0, 0.0, 0.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        // get intersent of sphere
        let sphere = Sphere::new();
        let xs = ray.intersect(&sphere);

        let unwrapped_xs = xs.as_ref().unwrap();

        // calculate potential hits on the optional value
        let sut = xs.hit();

        // is correct
        assert!(sut.is_some());
        let some = sut.unwrap();
        assert_eq!(some.at, 1.0);

        let object = match some.object {
            Object::Sphere(s) => s,
            _ => panic!(),
        };

        // hit references original sphere
        assert!(std::ptr::eq(object, &sphere));

        // can still access previously unwrapped elements
        assert_eq!(unwrapped_xs[0].at, -1.0);
        assert_eq!(unwrapped_xs[1].at, 1.0);

        let obj_from_unwrapped = match unwrapped_xs[0].object {
            Object::Sphere(o) => o,
            _ => panic!(),
        };
        assert!(std::ptr::eq(obj_from_unwrapped, &sphere));
    }
}
