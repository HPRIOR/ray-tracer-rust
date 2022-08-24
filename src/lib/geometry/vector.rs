
pub type Tup = (f64, f64, f64, f64);

// point = 1
pub fn point(x: f64, y: f64, z: f64) -> Tup {
    (x, y, z, 1.0)
}

// vector = 0
pub fn vector(x: f64, y: f64, z: f64) -> Tup {
    (x, y, z, 0.0)
}

pub trait Vector {
    type Output;
    fn length(self) -> f64;
    fn norm(self) -> Self::Output;
    fn dot(self, other: Self::Output) -> f64;
    fn cross_prod(self, other: Self::Output) -> Self::Output;
    fn x(self) -> f64;
    fn y(self) -> f64;
    fn z(self) -> f64;
}

pub trait Operations {
    type Output;
    fn add(self, rhs: Self::Output) -> Self::Output;
    fn sub(self, rhs: Self::Output) -> Self::Output;
    fn mul(self, rhs: f64) -> Self::Output;
    fn div(self, rhs: f64) -> Self::Output;
    fn neg(self) -> Self::Output;
}

trait Square {
    fn squared(self) -> f64;
}

impl Square for f64 {
    fn squared(self) -> f64 {
        self * self
    }
}

impl Vector for Tup {
    type Output = Tup;

    fn length(self) -> f64 {
        (self.0.squared() + self.1.squared() + self.2.squared()).sqrt()
    }

    fn norm(self) -> Self::Output {
        (
            self.0 / self.length(),
            self.1 / self.length(),
            self.2 / self.length(),
            self.3 / self.length(),
        )
    }

    fn dot(self, other: Self::Output) -> f64 {
        (self.0 * other.0) + (self.1 * other.1) + (self.2 * other.2) + (self.3 * self.3)
    }

    fn cross_prod(self, other: Self::Output) -> Self::Output {
        (
            (self.1 * other.2) - (self.2 * other.1),
            (self.2 * other.0) - (self.0 * other.2),
            (self.0 * other.1) - (self.1 * other.0),
            0.0,
        )
    }

    fn x(self) -> f64 {
        self.0
    }

    fn y(self) -> f64 {
        self.1
    }

    fn z(self) -> f64 {
        self.2
    }
}

impl Operations for Tup {
    type Output = Tup;

    fn add(self, rhs: Self::Output) -> Self::Output {
        (
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }

    fn sub(self, rhs: Self::Output) -> Self::Output {
        (
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }

    fn mul(self, rhs: f64) -> Self::Output {
        (self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }

    fn div(self, rhs: f64) -> Self::Output {
        (self.0 / rhs, self.1 / rhs, self.2 / rhs, self.3 / rhs)
    }

    fn neg(self) -> Self::Output {
        (-self.0, -self.1, -self.2, -self.3)
    }
}

#[cfg(test)]
mod tests {

    use super::{point, vector, Operations, Vector};

    #[test]
    fn vector_and_point_add_to_point() {
        let p1 = point(3.0, -2.0, 5.0);
        let v1 = vector(-2.0, 3.0, 1.0);
        let result = p1.add(v1);
        assert_eq!(result, point(1.0, 1.0, 6.0));
    }

    #[test]
    fn vector_and_vector_add_to_vector() {
        let v1 = vector(3.0, -2.0, 5.0);
        let v2 = vector(-2.0, 3.0, 1.0);
        let result = v1.add(v2);
        assert_eq!(result, vector(1.0, 1.0, 6.0))
    }

    #[test]
    fn point_and_point_subtract_to_vector() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        let result = p1.sub(p2);
        assert_eq!(result, vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn point_and_vector_subtract_to_point() {
        let v1 = vector(5.0, 6.0, 7.0);
        let p1 = point(3.0, 2.0, 1.0);
        let result = p1.sub(v1);
        assert_eq!(result, point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn vector_and_vector_subtract_to_vector() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        let result = v1.sub(v2);
        assert_eq!(result, vector(-2.0, -4.0, -6.0))
    }

    #[test]
    fn vector_can_be_negated() {
        let v1 = vector(3.0, 2.0, 1.0);
        let result = v1.neg();
        assert_eq!(result, vector(-3.0, -2.0, -1.0))
    }

    #[test]
    fn vector_and_vector_multiply_to_vector() {
        let v1 = vector(1.0, -2.0, 3.0);
        let result = v1.mul(3.5);
        assert_eq!(vector(3.5, -7.0, 10.5), result)
    }

    #[test]
    fn vector_and_vector_multiply_with_fraction() {
        let v1 = vector(1.0, -2.0, 3.0);
        let result = v1.mul(0.5);
        assert_eq!(vector(0.5, -1.0, 1.5), result)
    }

    #[test]
    fn vector_and_vector_divide_to_vector() {
        let v1 = vector(1.0, -2.0, 3.0);
        let result = v1.div(2.0);
        assert_eq!(vector(0.5, -1.0, 1.5), result)
    }

    #[test]
    fn vector_with_x_of_one_will_have_one_magniute() {
        let v1 = vector(1.0, 0.0, 0.0);
        let result = v1.length();
        assert_eq!(1.0, result)
    }

    #[test]
    fn vector_with_y_of_one_will_have_one_magniute() {
        let v1 = vector(0.0, 1.0, 0.0);
        let result = v1.length();
        assert_eq!(1.0, result)
    }

    #[test]
    fn vector_with_z_of_one_will_have_one_magniute() {
        let v1 = vector(0.0, 0.0, 1.0);
        let result = v1.length();
        assert_eq!(1.0, result)
    }

    #[test]
    fn vector_one_two_three_will_have_sqrt_14_magnitute() {
        let v1 = vector(1.0, 2.0, 3.0);
        let result = v1.length();
        assert_eq!(14.0_f64.sqrt(), result)
    }

    #[test]
    fn negative_vector_will_have_correct_magnitute() {
        let v1 = vector(-1.0, -2.0, -3.0);
        let result = v1.length();
        assert_eq!(14.0_f64.sqrt(), result)
    }

    #[test]
    fn norm_vector_of_x_4_will_have_x_1() {
        let v1 = vector(4.0, 0.0, 0.0);
        let result = v1.norm();
        assert_eq!(result, (1.0, 0.0, 0.0, 0.0))
    }

    #[test]
    fn complex_normalisation_is_correct() {
        let v1 = vector(1.0_f64, 2.0_f64, 3.0_f64);
        let result = v1.norm();
        assert_eq!(result, (0.2672612419124244, 0.5345224838248488, 0.8017837257372732, 0.0))
    }

    #[test]
    fn dot_of_two_simple_vectors_is_correct() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);
        assert_eq!(v1.dot(v2), 20.0)
    }

    #[test]
    fn cross_product_of_two_vectors_is_correct() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);
        assert_eq!(v1.cross_prod(v2), vector(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross_prod(v1), vector(1.0, -2.0, 1.0));
    }
}
