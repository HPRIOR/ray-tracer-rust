use crate::{colour::colour::Colour, geometry::vector::Tup, shapes::shape::TShape};

#[derive(Copy, Clone, Debug)]
pub struct StripePattern {
    a: Colour,
    b: Colour,
}

impl Default for StripePattern {
    fn default() -> Self {
        Self {
            a: Colour::white(),
            b: Colour::black(),
        }
    }
}

impl StripePattern {
    pub fn new(a: Colour, b: Colour) -> Self {
        Self { a, b }
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

    pub fn stripe_at_object(&self, object: &impl TShape, point: Tup) -> Colour {
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
            .build();

        let pattern = StripePattern::default();
        let colour = pattern.stripe_at_object(&object, point(1.5, 0.0, 0.0));
        assert_eq!(Colour::white(), colour);
    }
}
