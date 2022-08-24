#![allow(dead_code, unused_variables)]


use crate::{
    geometry::vector::{point, Operations, Tup, Vector},
    shapes::sphere::Sphere,
};

pub struct Ray {
    origin: Tup,
    direction: Tup
}

impl Ray {
    pub fn new(origin: Tup, direction: Tup) -> Self {
        Self { origin, direction }
    }

    fn position(&self, t: f64) -> Tup {
        self.direction.mul(t).add(self.origin)
    }

    fn intersect(&self, sphere: Sphere) -> Vec<f64> {
        let sphere_to_ray = self.origin.sub(point(0.0, 0.0, 0.0));
        let a = self.direction.dot(self.direction);
        let b = (self.direction.dot(sphere_to_ray)) * 2.0;
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        geometry::vector::{point, vector},
        shapes::sphere::Sphere,
    };

    use super::Ray;

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

        let xs = ray.intersect(sphere);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    }

    #[test]
    fn intersects_a_sphere_at_tengent() {
        let origin = point(0.0, 1.0, -5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere = Sphere::new();

        let xs = ray.intersect(sphere);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let origin = point(0.0, 2.0, -5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere = Sphere::new();

        let xs = ray.intersect(sphere);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_can_originate_inside_sphere() {
        let origin = point(0.0, 0.0, 0.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere = Sphere::new();

        let xs = ray.intersect(sphere);
        assert_eq!(xs.len(), 0);
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);
    }

    #[test]
    fn sphere_behind_ray_has_negative_intersect() {
        let origin = point(0.0, 0.0, 5.0);
        let direction = vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere = Sphere::new();

        let xs = ray.intersect(sphere);
        assert_eq!(xs.len(), 0);
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
    }
}
