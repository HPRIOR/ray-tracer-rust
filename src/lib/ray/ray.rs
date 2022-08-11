#![allow(dead_code)]

use num_traits::real::Real;

use crate::geometry::vector::{Tup, Operations};

pub struct Ray<T: Real> {
    origin: Tup<T>,
    direction: Tup<T>,
}

impl<T: Real> Ray<T> {
    pub fn new(origin: Tup<T>, direction: Tup<T>) -> Self {
        Self { origin, direction }
    }

    fn position(&self, t: T) -> Tup<T> {
        self.direction.mul(t).add(self.origin)
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::vector::{point, vector};

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
}
