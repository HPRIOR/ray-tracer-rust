use uuid::Uuid;

use crate::matrix::matrix::Matrix;

#[derive(Debug)]
pub struct Sphere {
    pub id: Uuid,
    pub transform: Matrix,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            transform: Matrix::ident(),
        }
    }

    pub fn with_transform(translation: Matrix) -> Self {
        Self {
            id: Uuid::new_v4(),
            transform: translation,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::matrix::Matrix;

    use super::Sphere;

    #[test]
    fn sphere_has_default_transformation() {
        let s = Sphere::new();
        assert_eq!(s.transform, Matrix::ident());
    }

    #[test]
    fn sphere_can_change_transformation() {
        let mut s = Sphere::new();
        let t = Matrix::translation(2.0, 3.0, 4.0);
        s.transform = t.clone();
        assert_eq!(s.transform, t);
    }

    #[test]
    fn sphere_can_be_created_with_new_transform() {
        let t = Matrix::translation(2.0, 3.0, 4.0);
        let s = Sphere::with_transform(t.clone());
        assert_eq!(s.transform, t);
    }
}
