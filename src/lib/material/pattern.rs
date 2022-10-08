use crate::{
    colour::colour::Colour, geometry::vector::Tup, matrix::matrix::Matrix, shapes::shape::TShape,
};

#[derive(Debug)]
pub struct StripePattern {
    a: Colour,
    b: Colour,
    transform: Matrix,
}

impl Default for StripePattern {
    fn default() -> Self {
        Self {
            a: Colour::white(),
            b: Colour::black(),
            transform: Matrix::default(),
        }
    }
}

impl StripePattern {
    pub fn new(a: Colour, b: Colour, transform: Matrix) -> Self {
        Self { a, b, transform }
    }

    pub fn stripe_at(&self, point: Tup) -> Colour {
        let x_i32 = point.0 as i32;
        let x_f64 = point.0;

        if x_i32 % 2 == 0 {
            if x_f64 >= 0.0 {
                self.a
            } else {
                self.b
            }
        } else {
            if x_f64 >= 0.0 {
                self.b
            } else {
                self.a
            }
        }
    }

    pub fn stripe_at_object(&self, object: Box<&dyn TShape>, world_point: Tup) -> Option<Colour> {
        object
            .transform()
            .inverse()
            .map(|m| m.mul_tup(world_point))
            .and_then(|o| self.transform.inverse().map(|p| p.mul_tup(o)))
            .map(|p| self.stripe_at(p))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        colour::colour::Colour, geometry::vector::point, matrix::matrix::Matrix,
        shapes::sphere::Sphere,
    };

    use super::StripePattern;

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = StripePattern::default();
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.stripe_at(point(0.0, 1.0, 0.0)), Colour::white());
        assert_eq!(pattern.stripe_at(point(0.0, 2.0, 0.0)), Colour::white());
    }
    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = StripePattern::default();
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 1.0)), Colour::white());
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 2.0)), Colour::white());
    }
    #[test]
    fn stripe_pattern_is_alternates_on_x() {
        let pattern = StripePattern::default();
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.stripe_at(point(0.9, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.stripe_at(point(1.0, 0.0, 0.0)), Colour::black());
        assert_eq!(pattern.stripe_at(point(-0.1, 0.0, 0.0)), Colour::black());
        assert_eq!(pattern.stripe_at(point(-0.9, 0.0, 0.0)), Colour::black());
        assert_eq!(pattern.stripe_at(point(-1.1, 0.0, 0.0)), Colour::white());
    }

    #[test]
    fn stripes_with_object_transformation() {
        let object = Sphere::builder()
            .with_transform(Matrix::scaling(2.0, 2.0, 2.0))
            .build_trait();

        let pattern = StripePattern::default();
        let colour = pattern.stripe_at_object(object.to_trait_ref(), point(1.5, 0.0, 0.0));
        assert_eq!(Colour::white(), colour.unwrap());
    }

    #[test]
    fn stripes_with_pattern_transformation() {
        let object = Sphere::builder()
            .with_transform(Matrix::scaling(2.0, 2.0, 2.0))
            .build_trait();

        let pattern = StripePattern::new(
            Colour::white(),
            Colour::black(),
            Matrix::scaling(2.0, 2.0, 2.0),
        );
        let colour = pattern.stripe_at_object(object.to_trait_ref(), point(1.5, 0.0, 0.0));
        assert_eq!(Colour::white(), colour.unwrap());
    }

    #[test]
    fn stripes_with_pattern_and_object_transformation() {
        let object = Sphere::builder()
            .with_transform(Matrix::scaling(2.0, 2.0, 2.0))
            .build_trait();

        let pattern = StripePattern::new(
            Colour::white(),
            Colour::black(),
            Matrix::translation(0.5, 0.0, 0.0),
        );
        let colour = pattern.stripe_at_object(object.to_trait_ref(), point(2.5, 0.0, 0.0));
        assert_eq!(Colour::white(), colour.unwrap());
    }
}
