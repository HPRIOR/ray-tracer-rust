use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Colour {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Colour {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    pub fn black() -> Self {
        Self::default()
    }
    pub fn white() -> Self {
        Self {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
    }
}
impl Default for Colour {
    fn default() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }
}

impl Add for Colour {
    type Output = Colour;

    fn add(self, rhs: Self) -> Self::Output {
        Colour {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Add<f64> for Colour {
    type Output = Colour;

    fn add(self, rhs: f64) -> Self::Output {
        Colour {
            red: self.red + rhs,
            green: self.green + rhs,
            blue: self.blue + rhs,
        }
    }
}

impl Sub for Colour {
    type Output = Colour;

    fn sub(self, rhs: Self) -> Self::Output {
        Colour {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl Mul<f64> for Colour {
    type Output = Colour;

    fn mul(self, rhs: f64) -> Self::Output {
        Colour {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Mul<Colour> for Colour {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Self::Output {
        Colour {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Colour;
    use float_cmp::approx_eq;

    #[test]
    pub fn constructor_works() {
        let c = Colour::new(1.0, 2.0, 3.0);
        assert_eq!(c.red, 1.0);
        assert_eq!(c.green, 2.0);
        assert_eq!(c.blue, 3.0);
    }

    #[test]
    pub fn can_add_two_colours() {
        let c1 = Colour::new(0.9, 0.6, 0.75);
        let c2 = Colour::new(0.7, 0.1, 0.25);
        let sut = c1 + c2;
        assert!(approx_eq!(f64, sut.red, 1.6, ulps = 2));
        assert!(approx_eq!(f64, sut.green, 0.7, ulps = 2));
        assert!(approx_eq!(f64, sut.blue, 1.0, ulps = 2));
    }

    #[test]
    pub fn can_subtract_two_colours() {
        let c1 = Colour::new(0.9, 0.6, 0.75);
        let c2 = Colour::new(0.7, 0.1, 0.25);
        let sut = c1 - c2;
        assert!(approx_eq!(f64, sut.red, 0.2, ulps = 2));
        assert!(approx_eq!(f64, sut.green, 0.5, ulps = 2));
        assert!(approx_eq!(f64, sut.blue, 0.5, ulps = 2));
    }

    #[test]
    pub fn can_multiply_by_scalar() {
        let c1 = Colour::new(0.2, 0.3, 0.4);
        let sut = c1 * 2.0;
        assert_eq!(sut, Colour::new(0.4, 0.6, 0.8));
    }

    #[test]
    pub fn can_multiply_by_another_colour() {
        let c1 = Colour::new(1.0, 0.2, 0.4);
        let c2 = Colour::new(0.9, 1.0, 0.1);
        let sut = c1 * c2;
        assert!(approx_eq!(f64, sut.red, 0.9, ulps = 2));
        assert!(approx_eq!(f64, sut.green, 0.2, ulps = 2));
        assert!(approx_eq!(f64, sut.blue, 0.04, ulps = 2));
    }
}
