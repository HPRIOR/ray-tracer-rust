use crate::{colour::colour::Colour, geometry::vector::Tup};

struct Pattern {
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
    fn new(a: Colour, b: Colour) -> Self {
        Self { a, b }
    }

    fn stripe_at(&self, point: Tup) -> Colour{
        todo!()
    }
}

#[cfg(test)]
mod tests{
    use super::Pattern;

    #[test]
    fn stripe_pattern_is_constant_in_y(){
        let pattern = Pattern::default();
        assert_eq!()

    }
}



