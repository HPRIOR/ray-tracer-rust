use std::ops::Add;

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
}

impl Add for Vector{
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: 0.0
        }      
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
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: 1.0,
        }
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
}
