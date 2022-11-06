#![allow(unused)]
use std::fmt::Debug;

use crate::{
    colour::colour::Colour, geometry::vector::Tup, matrix::matrix::Matrix, shapes::shape::TShape,
};

pub trait TPattern: Send + Sync + Debug {
    fn transform(&self) -> &Matrix;
    fn pattern_at(&self, point: Tup) -> Colour;
    fn pattern_at_object(&self, object: Box<&dyn TShape>, world_point: Tup) -> Option<Colour> {
        object
            .transform()
            .inverse()
            .map(|m| m.mul_tup(world_point))
            .and_then(|o| self.transform().inverse().map(|p| p.mul_tup(o)))
            .map(|p| self.pattern_at(p))
    }
}

/// --- Stripe --- ///

#[derive(Debug, Clone)]
pub struct Stripe {
    a: Colour,
    b: Colour,
    transform: Matrix,
}

impl TPattern for Stripe {
    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn pattern_at(&self, point: Tup) -> Colour {
        let check = point.0.floor() % 2.0 == 0.0;
        if check {
            self.a
        } else {
            self.b
        }
    }
}

impl Default for Stripe {
    fn default() -> Self {
        Self {
            a: Colour::white(),
            b: Colour::black(),
            transform: Matrix::default(),
        }
    }
}

impl Stripe {
    pub fn new(a: Colour, b: Colour, transform: Matrix) -> Self {
        Self { a, b, transform }
    }
}

/// --- Gradient --- ///

#[derive(Debug, Clone)]
pub struct Gradient {
    a: Colour,
    b: Colour,
    transform: Matrix,
}

impl Default for Gradient {
    fn default() -> Self {
        Self {
            a: Colour::white(),
            b: Colour::black(),
            transform: Default::default(),
        }
    }
}

impl TPattern for Gradient {
    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn pattern_at(&self, point: Tup) -> Colour {
        let distance = self.b - self.a;
        let fraction = point.0 - point.0.floor();
        self.a + distance * fraction
    }
}

impl Gradient {
    fn new(a: Colour, b: Colour, transform: Matrix) -> Self {
        Self { a, b, transform }
    }
}
/// --- Ring --- ///

#[derive(Debug, Clone)]
pub struct Ring {
    a: Colour,
    b: Colour,
    transform: Matrix,
}

impl Default for Ring {
    fn default() -> Self {
        Self {
            a: Colour::white(),
            b: Colour::black(),
            transform: Default::default(),
        }
    }
}

impl TPattern for Ring {
    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn pattern_at(&self, point: Tup) -> Colour {
        let check = ((point.0 + point.2).sqrt().floor() % 2.0) == 0.0;
        if check {
            self.a
        } else {
            self.b
        }
    }
}

impl Ring {
    pub fn new(a: Colour, b: Colour, transform: Matrix) -> Self {
        Self { a, b, transform }
    }
}
/// --- Checker --- ///

#[derive(Debug, Clone)]
pub struct Checker {
    a: Colour,
    b: Colour,
    transform: Matrix,
}

impl Default for Checker {
    fn default() -> Self {
        Self {
            a: Colour::white(),
            b: Colour::black(),
            transform: Default::default(),
        }
    }
}

impl TPattern for Checker {
    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn pattern_at(&self, point: Tup) -> Colour {
        let check = (point.0.floor() + point.1.floor() + point.2.floor()) % 2.0 == 0.0;
        if check {
            self.a
        } else {
            self.b
        }
    }
}

impl Checker {
    fn new(a: Colour, b: Colour, transform: Matrix) -> Self {
        Self { a, b, transform }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        colour::colour::Colour,
        geometry::vector::point,
        material::pattern::{Checker, Ring, TPattern},
        matrix::matrix::Matrix,
        shapes::{shape::TShapeBuilder, sphere::Sphere},
    };

    use super::{Gradient, Stripe};

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = Stripe::default();
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(0.0, 1.0, 0.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(0.0, 2.0, 0.0)), Colour::white());
    }
    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = Stripe::default();
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 1.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 2.0)), Colour::white());
    }
    #[test]
    fn stripe_pattern_is_alternates_on_x() {
        let pattern = Stripe::default();
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(0.9, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(1.0, 0.0, 0.0)), Colour::black());
        assert_eq!(pattern.pattern_at(point(-0.1, 0.0, 0.0)), Colour::black());
        assert_eq!(pattern.pattern_at(point(-0.9, 0.0, 0.0)), Colour::black());
        assert_eq!(pattern.pattern_at(point(-1.1, 0.0, 0.0)), Colour::white());
    }

    #[test]
    fn stripes_with_object_transformation() {
        let object = Sphere::builder()
            .with_transform(Matrix::scaling(2.0, 2.0, 2.0))
            .build_trait();

        let pattern = Stripe::default();
        let colour = pattern.pattern_at_object(object.to_trait_ref(), point(1.5, 0.0, 0.0));
        assert_eq!(Colour::white(), colour.unwrap());
    }

    #[test]
    fn stripes_with_pattern_transformation() {
        let object = Sphere::builder()
            .with_transform(Matrix::scaling(2.0, 2.0, 2.0))
            .build_trait();

        let pattern = Stripe::new(
            Colour::white(),
            Colour::black(),
            Matrix::scaling(2.0, 2.0, 2.0),
        );
        let colour = pattern.pattern_at_object(object.to_trait_ref(), point(1.5, 0.0, 0.0));
        assert_eq!(Colour::white(), colour.unwrap());
    }

    #[test]
    fn stripes_with_pattern_and_object_transformation() {
        let object = Sphere::builder()
            .with_transform(Matrix::scaling(2.0, 2.0, 2.0))
            .build_trait();

        let pattern = Stripe::new(
            Colour::white(),
            Colour::black(),
            Matrix::translation(0.5, 0.0, 0.0),
        );
        let colour = pattern.pattern_at_object(object.to_trait_ref(), point(2.5, 0.0, 0.0));
        assert_eq!(Colour::white(), colour.unwrap());
    }
    #[test]
    fn gradient_linearly_interpolates_between_colours() {
        let pattern = Gradient::default();
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Colour::white());
        assert_eq!(
            pattern.pattern_at(point(0.25, 0.0, 0.0)),
            Colour::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.pattern_at(point(0.5, 0.0, 0.0)),
            Colour::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.pattern_at(point(0.75, 0.0, 0.0)),
            Colour::new(0.25, 0.25, 0.25)
        );
    }
    #[test]
    fn ring_should_extend_both_x_and_z() {
        let pattern = Ring::default();
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(1.0, 0.0, 0.0)), Colour::black());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 1.0)), Colour::black());
        assert_eq!(
            pattern.pattern_at(point(0.708, 0.0, 0.708)),
            Colour::black()
        );
    }
    #[test]
    fn checker_should_repeat_in_x() {
        let pattern = Checker::default();
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(0.99, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(1.01, 0.0, 0.0)), Colour::black());
    }
    #[test]
    fn checker_should_repeat_in_y() {
        let pattern = Checker::default();
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(0.0, 0.99, 0.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(0.0, 1.01, 0.0)), Colour::black());
    }
    #[test]
    fn checker_should_repeat_in_z() {
        let pattern = Checker::default();
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.99)), Colour::white());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 1.01)), Colour::black());
    }
}
