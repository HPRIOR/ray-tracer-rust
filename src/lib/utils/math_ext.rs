pub trait Square {
    fn squared(self) -> f64;
}

impl Square for f64 {
    fn squared(self) -> f64 {
        self * self
    }
}
