use std::ops::{Add, Div, Mul, Neg, Sub};

use num_traits::Num;

pub trait HasCoordinates {
    fn get_coords(&self) -> (f32, f32, f32);
    fn get_usize_coord(&self) -> (usize, usize, usize);
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    w: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn length(self) -> f32 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn norm(self) -> Vector {
        Vector::new(
            self.x / self.length(),
            self.y / self.length(),
            self.z / self.length(),
        )
    }

    pub fn dot(self, other: Vector) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn cross_prod(self, other: Vector) -> Vector {
        Vector::new(
            (self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x),
        )
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Vector{
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(0.0 - self.x, 0.0 - self.y, 0.0 - self.z)
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    w: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    #[allow(dead_code)]
    fn length(self) -> f32 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    #[allow(dead_code)]
    fn norm(self) -> Vector {
        Vector::new(
            self.x / self.length(),
            self.y / self.length(),
            self.z / self.length(),
        )
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
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
        let v1 = Vector::new(1.0, 2.0, 3.0);
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
