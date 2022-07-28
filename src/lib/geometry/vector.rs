use std::ops::{Add, Div, Mul, Neg, Sub};

use num_traits::{real::Real, One, Zero};

pub trait HasCoordinates<T: Real> {
    fn get_coords(&self) -> (T, T, T, T);

    fn get_usize_coord(&self) -> (usize, usize, usize) {
        let (x, y, z, _) = self.get_coords();
        (
            x.to_usize().unwrap(),
            y.to_usize().unwrap(),
            z.to_usize().unwrap(),
        )
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector<T: Real> {
    pub x: T,
    pub y: T,
    pub z: T,
    w: T,
}

impl<T: Real> Vector<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z,
            w: Zero::zero(),
        }
    }

    pub fn length(self) -> T {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn norm(self) -> Vector<T> {
        Vector::new(
            self.x / self.length(),
            self.y / self.length(),
            self.z / self.length(),
        )
    }

    pub fn dot(self, other: Vector<T>) -> T {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn cross_prod(self, other: Vector<T>) -> Vector<T> {
        Vector::new(
            (self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x),
        )
    }
}

impl<T: Real> Add for Vector<T> {
    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Real> Sub for Vector<T> {
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Real> Neg for Vector<T> {
    type Output = Vector<T>;

    fn neg(self) -> Self::Output {
        Vector::new(self.x.neg(), self.y.neg(), self.z.neg())
    }
}

impl<T: Real> Div<T> for Vector<T> {
    type Output = Vector<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T: Real> Mul<T> for Vector<T> {
    type Output = Vector<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T:Real> HasCoordinates<T>  for Vector<T>{
    fn get_coords(&self) -> (T, T, T, T) {
        (self.x, self.y, self.z, self.w)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point<T: Real> {
    pub x: T,
    pub y: T,
    pub z: T,
    w: T,
}

impl<T: Real> Point<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z,
            w: One::one(),
        }
    }

    #[allow(dead_code)]
    fn length(self) -> T {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    #[allow(dead_code)]
    fn norm(self) -> Vector<T> {
        Vector::new(
            self.x / self.length(),
            self.y / self.length(),
            self.z / self.length(),
        )
    }
}

impl<T: Real> Add<Vector<T>> for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Real> Sub for Point<T> {
    type Output = Vector<T>;

    fn sub(self, rhs: Point<T>) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Real> Sub<Vector<T>> for Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T:Real> HasCoordinates<T>  for Point<T>{
    fn get_coords(&self) -> (T, T, T, T) {
        (self.x, self.y, self.z, self.w)
    }
} 

#[cfg(test)]
mod tests {
    use super::{Point, Vector};

    #[test]
    fn vector_and_point_add_to_point() {
        let point = Point::new(3.0, -2.0, 5.0);
        let vector = Vector::new(-2.0, 3.0, 1.0);
        let result = point + vector;
        assert_eq!(result, Point::new(1.0, 1.0, 6.0))
    }

    #[test]
    fn vector_and_vector_add_to_vector() {
        let v1 = Vector::new(3.0, -2.0, 5.0);
        let v2 = Vector::new(-2.0, 3.0, 1.0);
        let result = v1 + v2;
        assert_eq!(result, Vector::new(1.0, 1.0, 6.0))
    }

    #[test]
    fn point_and_point_subtract_to_vector() {
        let v1 = Point::new(3.0, 2.0, 1.0);
        let v2 = Point::new(5.0, 6.0, 7.0);
        let result = v1 - v2;
        assert_eq!(result, Vector::new(-2.0, -4.0, -6.0))
    }

    #[test]
    fn point_and_vector_subtract_to_point() {
        let v1 = Vector::new(5.0, 6.0, 7.0);
        let p1 = Point::new(3.0, 2.0, 1.0);
        let result = p1 - v1;
        assert_eq!(result, Point::new(-2.0, -4.0, -6.0))
    }

    #[test]
    fn vector_and_vector_subtract_to_vector() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);
        let result = v1 - v2;
        assert_eq!(result, Vector::new(-2.0, -4.0, -6.0))
    }

    #[test]
    fn vector_can_be_negated() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let result = -v1;
        assert_eq!(result, Vector::new(-3.0, -2.0, -1.0))
    }

    #[test]
    fn vector_and_vector_multiply_to_vector() {
        let v1 = Vector::new(1.0, -2.0, 3.0);
        let result = v1 * 3.5;
        assert_eq!(Vector::new(3.5, -7.0, 10.5), result)
    }

    #[test]
    fn vector_and_vector_multiply_with_fraction() {
        let v1 = Vector::new(1.0, -2.0, 3.0);
        let result = v1 * 0.5;
        assert_eq!(Vector::new(0.5, -1.0, 1.5), result)
    }

    #[test]
    fn vector_and_vector_divide_to_vector() {
        let v1 = Vector::new(1.0, -2.0, 3.0);
        let result = v1 / 2.0;
        assert_eq!(Vector::new(0.5, -1.0, 1.5), result)
    }

    #[test]
    fn vector_with_x_of_one_will_have_one_magniute() {
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let result = v1.length();
        assert_eq!(1.0, result)
    }

    #[test]
    fn vector_with_y_of_one_will_have_one_magniute() {
        let v1 = Vector::new(0.0, 1.0, 0.0);
        let result = v1.length();
        assert_eq!(1.0, result)
    }

    #[test]
    fn vector_with_z_of_one_will_have_one_magniute() {
        let v1 = Vector::new(0.0, 0.0, 1.0);
        let result = v1.length();
        assert_eq!(1.0, result)
    }

    #[test]
    fn vector_one_two_three_will_have_sqrt_14_magnitute() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let result = v1.length();
        assert_eq!(14.0_f32.sqrt(), result)
    }

    #[test]
    fn negative_vector_will_have_correct_magnitute() {
        let v1 = Vector::new(-1.0, -2.0, -3.0);
        let result = v1.length();
        assert_eq!(14.0_f32.sqrt(), result)
    }

    #[test]
    fn norm_vector_of_x_4_will_have_x_1() {
        let v1 = Vector::new(4.0, 0.0, 0.0);
        let result = v1.norm();
        assert_eq!(result, Vector::new(1.0, 0.0, 0.0))
    }

    #[test]
    fn complex_normalisation_is_correct() {
        let v1 = Vector::new(1.0_f32, 2.0_f32, 3.0_f32);
        let result = v1.norm();
        assert_eq!(result, Vector::new(0.26726124, 0.5345225, 0.8017837))
    }

    #[test]
    fn dot_of_two_simple_vectors_is_correct() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(v1.dot(v2), 20.0)
    }

    #[test]
    fn cross_product_of_two_vectors_is_correct() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(v1.cross_prod(v2), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross_prod(v1), Vector::new(1.0, -2.0, 1.0));
    }
}
