use crate::{colour::colour::Colour, geometry::vector::Tup};

pub struct Pattern {
    a: Colour,
    b: Colour,
}

impl Default for Pattern {
    fn default() -> Self {
        Self {
            a: Colour::white(),
            b: Colour::black(),
        }
    }
}

impl Pattern {
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
}

#[cfg(test)]
mod tests {
    use crate::{colour::colour::Colour, geometry::vector::point};

    use super::Pattern;

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = Pattern::default();
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.stripe_at(point(0.0, 1.0, 0.0)), Colour::white());
        assert_eq!(pattern.stripe_at(point(0.0, 2.0, 0.0)), Colour::white());
    }
    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = Pattern::default();
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 1.0)), Colour::white());
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 2.0)), Colour::white());
    }
    #[test]
    fn stripe_pattern_is_alternates_on_x() {
        let pattern = Pattern::default();
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.stripe_at(point(0.9, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.stripe_at(point(1.0, 0.0, 0.0)), Colour::black());
        assert_eq!(pattern.stripe_at(point(-0.1, 0.0, 0.0)), Colour::black());
        assert_eq!(pattern.stripe_at(point(-0.9, 0.0, 0.0)), Colour::black());
        assert_eq!(pattern.stripe_at(point(-1.1, 0.0, 0.0)), Colour::white());
    }
}
