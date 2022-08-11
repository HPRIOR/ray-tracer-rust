use num_traits::{One, Zero, real::Real};

pub type Tup<T> = (T, T, T, T);

// point = 1
// vector = 0
pub fn point<T: Real>(x: T, y: T, z: T) -> Tup<T> {
    (x, y, z, One::one())
}

pub fn vector<T: Real>(x: T, y: T, z: T) -> Tup<T> {
    (x, y, z, Zero::zero())
}

pub trait Vector<T: Real> {
    type Output;
    fn length(self) -> T;
    fn norm(self) -> Self::Output;
    fn dot(self, other: Self::Output) -> T;
    fn cross_prod(self, other: Self::Output) -> Self::Output;
    fn x(self) -> T;
    fn y(self) -> T;
    fn z(self) -> T;
}

pub trait Operations<T: Real> {
    type Output;
    fn add(self, rhs: Self::Output) -> Self::Output;
    fn sub(self, rhs: Self::Output) -> Self::Output;
    fn mul(self, rhs: T) -> Self::Output;
    fn div(self, rhs: T) -> Self::Output;
    fn neg(self) -> Self::Output;
}

trait Square<T> {
    fn squared(self) -> T;
}

impl<T: Real> Square<T> for T {
    fn squared(self) -> T {
        self * self
    }
}

impl<T: Real> Vector<T> for Tup<T> {
    type Output = Tup<T>;

    fn length(self) -> T {
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

    fn dot(self, other: Self::Output) -> T {
        (self.0 * other.0) + (self.1 * other.1) + (self.2 * other.2) + (self.3 * self.3)
    }

    fn cross_prod(self, other: Self::Output) -> Self::Output {
        (
            (self.1 * other.2) - (self.2 * other.1),
            (self.2 * other.0) - (self.0 * other.2),
            (self.0 * other.1) - (self.1 * other.0),
            Zero::zero(),
        )
    }

    fn x(self) -> T {
        self.0
    }

    fn y(self) -> T {
        self.1
    }

    fn z(self) -> T {
        self.2
    }
}

impl<T: Real> Operations<T> for Tup<T> {
    type Output = Tup<T>;

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

    fn mul(self, rhs: T) -> Self::Output {
        (self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }

    fn div(self, rhs: T) -> Self::Output {
        (self.0 / rhs, self.1 / rhs, self.2 / rhs, self.3 / rhs)
    }

    fn neg(self) -> Self::Output {
        (self.0.neg(), self.1.neg(), self.2.neg(), self.3.neg())
    }
}

#[cfg(test)]
mod tests {

    use super::{Operations, Vector, vector, point};

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
        assert_eq!(14.0_f32.sqrt(), result)
    }

    #[test]
    fn negative_vector_will_have_correct_magnitute() {
        let v1 = vector(-1.0, -2.0, -3.0);
        let result = v1.length();
        assert_eq!(14.0_f32.sqrt(), result)
    }

    #[test]
    fn norm_vector_of_x_4_will_have_x_1() {
        let v1 = vector(4.0, 0.0, 0.0);
        let result = v1.norm();
        assert_eq!(result, (1.0, 0.0, 0.0, 0.0))
    }

    #[test]
    fn complex_normalisation_is_correct() {
        let v1 = vector(1.0_f32, 2.0_f32, 3.0_f32);
        let result = v1.norm();
        assert_eq!(result, (0.26726124, 0.5345225, 0.8017837, 0.0))
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
