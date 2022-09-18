#![allow(dead_code,unused_variables)]

use crate::geometry::vector::{Operations, Tup, Vector};

type MatrixVec = Vec<Vec<f64>>;

#[derive(PartialEq, Debug, Clone)]
pub struct Matrix {
    matrix: MatrixVec,
}

pub enum Axis {
    X,
    Y,
    Z,
}

impl Matrix {
    pub fn new(matrix: MatrixVec) -> Self {
        Self { matrix }
    }

    pub fn len(&self) -> usize {
        self.matrix[0].len()
    }

    pub fn ident() -> Self {
        Self {
            matrix: vec![
                vec![1.0, 0.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0, 0.0],
                vec![0.0, 0.0, 1.0, 0.0],
                vec![0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    fn view_transform(from: Tup, to: Tup, up: Tup) -> Self {
        let forward = (to.sub(from)).norm();
        let upn = up.norm();
        let left = forward.cross_prod(upn);
        let true_up = left.cross_prod(forward);
        let orientation = Self {
            matrix: vec![
                vec![left.0, left.1, left.2, 0.0],
                vec![true_up.0, true_up.1, true_up.2, 0.0],
                vec![-forward.0, -forward.1, -forward.2, 0.0],
                vec![0.0, 0.0, 0.0, 1.0],
            ],
        };
        orientation.mul(&Matrix::translation(-from.0, -from.1, -from.2))
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        Self {
            matrix: vec![
                vec![x, 0.0, 0.0, 0.0],
                vec![0.0, y, 0.0, 0.0],
                vec![0.0, 0.0, z, 0.0],
                vec![0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn scale(&self, x: f64, y: f64, z: f64) -> Self {
        Matrix::scaling(x, y, z).mul(&self)
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        Self {
            matrix: vec![
                vec![1.0, 0.0, 0.0, x],
                vec![0.0, 1.0, 0.0, y],
                vec![0.0, 0.0, 1.0, z],
                vec![0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn translate(&self, x: f64, y: f64, z: f64) -> Self {
        Matrix::translation(x, y, z).mul(&self)
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.matrix[row][col]
    }

    pub fn transpose(&self) -> Self {
        let matrix = &self.matrix;

        let mut new_matrix = self.matrix.clone();

        for (i, row) in matrix.into_iter().enumerate() {
            for (j, col) in row.into_iter().enumerate() {
                new_matrix[j][i] = *col;
            }
        }

        return Matrix::new(new_matrix);
    }

    fn determinant(&self) -> f64 {
        let matrix = &self.matrix;
        if matrix.len() == 2 {
            return (matrix[0][0] * matrix[1][1]) - (matrix[0][1] * matrix[1][0]);
        }

        matrix[0]
            .iter()
            .enumerate()
            .fold(0.0, |acc, (i, x)| (acc + *x * self.cofactor(0, i)))
    }

    fn sub(&self, row_size: usize, col_size: usize) -> Self {
        Matrix::new(
            self.matrix
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != row_size)
                .map(|(_, row)| {
                    row.into_iter()
                        .enumerate()
                        .filter(|(j, _)| *j != col_size)
                        .map(|(_, col)| *col)
                        .collect()
                })
                .collect(),
        )
    }

    fn minor(&self, row_size: usize, col_size: usize) -> f64 {
        let sub_matrix = &self.sub(row_size, col_size);
        sub_matrix.determinant()
    }

    fn cofactor(&self, row_size: usize, col_size: usize) -> f64 {
        let minor = self.minor(row_size, col_size);
        if (row_size + col_size) % 2 == 0 {
            minor
        } else {
            let zero = 0.0;
            zero - minor
        }
    }

    pub fn inverse(&self) -> Option<Self> {
        if self.determinant() == 0.0 {
            None
        } else {
            let length = self.matrix.len();
            let cofactors: Matrix = Matrix::new(
                (0..length)
                    .map(|i| (0..length).map(|j| self.cofactor(i, j)).collect())
                    .collect(),
            );

            let determinant = &self.determinant();
            let transposed = cofactors.transpose();
            Some(Matrix::new(
                transposed
                    .matrix
                    .into_iter()
                    .map(|row| row.into_iter().map(|col| col / *determinant).collect())
                    .collect(),
            ))
        }
    }

    fn mul(&self, rhs: &Matrix) -> Self {
        let length = self.matrix.len();
        let width = rhs.matrix[0].len();

        Matrix::new(
            (0..length)
                .map(|i| {
                    (0..width)
                        .map(move |j| (i, j))
                        .map(|(i, j)| {
                            (0..length)
                                .fold(0.0, |acc, x| acc + self.matrix[i][x] * rhs.matrix[x][j])
                        })
                        .collect()
                })
                .collect(),
        )
    }

    pub fn mul_tup(&self, rhs: Tup) -> Tup {
        fn multiply_row(row: &Vec<f64>, tuple: Tup) -> f64 {
            row[0] * tuple.0 + row[1] * tuple.1 + row[2] * tuple.2 + row[3] * tuple.3
        }

        (
            multiply_row(&self.matrix[0], rhs),
            multiply_row(&self.matrix[1], rhs),
            multiply_row(&self.matrix[2], rhs),
            multiply_row(&self.matrix[3], rhs),
        )
    }

    fn rotation(around: Axis, radians: f64) -> Self {
        match around {
            Axis::X => Self {
                matrix: vec![
                    vec![1.0, 0.0, 0.0, 0.0],
                    vec![0.0, radians.cos(), -radians.sin(), 0.0],
                    vec![0.0, radians.sin(), radians.cos(), 0.0],
                    vec![0.0, 0.0, 0.0, 1.0],
                ],
            },
            Axis::Y => Self {
                matrix: vec![
                    vec![radians.cos(), 0.0, radians.sin(), 0.0],
                    vec![0.0, 1.0, 0.0, 0.0],
                    vec![-radians.sin(), 0.0, radians.cos(), 0.0],
                    vec![0.0, 0.0, 0.0, 1.0],
                ],
            },
            Axis::Z => Self {
                matrix: vec![
                    vec![radians.cos(), -radians.sin(), 0.0, 0.0],
                    vec![radians.sin(), radians.cos(), 0.0, 0.0],
                    vec![0.0, 0.0, 1.0, 0.0],
                    vec![0.0, 0.0, 0.0, 1.0],
                ],
            },
        }
    }

    pub fn rotate(&self, around: Axis, radians: f64) -> Self {
        Matrix::rotation(around, radians).mul(&self)
    }

    fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Self {
            matrix: vec![
                vec![1.0, xy, xz, 0.0],
                vec![yx, 1.0, yz, 0.0],
                vec![zx, zy, 1.0, 0.0],
                vec![0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn shear(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Matrix::shearing(xy, xz, yx, yz, zx, zy).mul(&self)
    }
}

#[cfg(test)]
mod tests {

    use std::f64::consts::PI;

    use float_cmp::approx_eq;
    use num_traits::Float;

    use crate::{
        geometry::vector::{point, vector},
        utils::test::ApproxEq,
    };

    use super::{Axis, Matrix};

    #[test]
    fn matrix_elements_are_correct() {
        let matrix: Matrix = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5],
        ]);
        assert_eq!(matrix.get(0, 0), 1.0);
        assert_eq!(matrix.get(0, 3), 4.0);
        assert_eq!(matrix.get(1, 0), 5.5);
        assert_eq!(matrix.get(1, 2), 7.5);
        assert_eq!(matrix.get(2, 2), 11.0);
        assert_eq!(matrix.get(3, 0), 13.5);
        assert_eq!(matrix.get(3, 2), 15.5);
    }

    #[test]
    fn can_represent_two_by_two_matrix() {
        let matrix: Matrix = Matrix::new(vec![vec![-3.0, 5.0], vec![1.0, -2.0]]);
        assert_eq!(matrix.get(0, 0), -3.0);
        assert_eq!(matrix.get(0, 1), 5.0);
        assert_eq!(matrix.get(1, 0), 1.0);
        assert_eq!(matrix.get(1, 1), -2.0);
    }

    #[test]
    fn can_represent_three_by_three_matrix() {
        let matrix: Matrix = Matrix::new(vec![
            vec![-3.0, 5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0],
        ]);
        assert_eq!(matrix.get(0, 0), -3.0);
        assert_eq!(matrix.get(0, 1), 5.0);
        assert_eq!(matrix.get(0, 2), 0.0);
        assert_eq!(matrix.get(1, 0), 1.0);
        assert_eq!(matrix.get(1, 1), -2.0);
        assert_eq!(matrix.get(1, 2), -7.0);
        assert_eq!(matrix.get(2, 0), 0.0);
        assert_eq!(matrix.get(2, 1), 1.0);
        assert_eq!(matrix.get(2, 2), 1.0);
    }

    #[test]
    fn matrix_are_equal() {
        let m1: Matrix = Matrix::new(vec![
            vec![-3.0, 5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0],
        ]);
        let m2: Matrix = Matrix::new(vec![
            vec![-3.0, 5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0],
        ]);
        let sut = m1 == m2;
        assert!(sut);
    }

    #[test]
    fn matrix_are_ne() {
        let m1: Matrix = Matrix::new(vec![
            vec![-3.0, 5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0],
        ]);
        let m2: Matrix = Matrix::new(vec![
            vec![-3.0, 6.0, 0.0],
            vec![2.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0],
        ]);
        let sut = m1 != m2;
        assert!(sut);
    }

    #[test]
    fn matrix_can_be_multiplied_together() {
        let m1: Matrix = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let m2: Matrix = Matrix::new(vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ]);
        let sut = m1.mul(&m2);
        let expected: Matrix = Matrix::new(vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0],
        ]);
        assert_eq!(sut, expected);
    }

    #[test]
    fn matrix_can_be_multiplied_by_tuple() {
        let matrix: Matrix = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);

        let tuple = (1.0, 2.0, 3.0, 1.0);

        let sut = matrix.mul_tup(tuple);
        let expected = (18.0, 24.0, 33.0, 1.0);
        assert_eq!(sut, expected);
    }

    #[test]
    fn matrix_multiplied_by_identity_produces_original() {
        let matrix: Matrix = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);

        let sut = matrix.mul(&Matrix::ident());

        assert_eq!(sut, matrix);
    }

    #[test]
    fn matrix_transposes_correctly() {
        let matrix: Matrix = Matrix::new(vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        ]);

        let sut = matrix.transpose();
        let expected = Matrix::new(vec![
            vec![0.0, 9.0, 1.0, 0.0],
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0],
        ]);

        assert_eq!(sut, expected);
    }

    #[test]
    fn transpose_of_identity_is_identity() {
        let ident: Matrix = Matrix::ident();
        let sut = ident.transpose();
        assert_eq!(sut, ident)
    }

    #[test]
    fn determinant_base_is_correct() {
        let matrix = Matrix::new(vec![vec![1.0, 5.0], vec![-3.0, 2.0]]);
        let sut = matrix.determinant();
        assert_eq!(sut, 17.0);
    }

    #[test]
    fn sub_matrix_of_three_by_three_is_two_by_two() {
        let matrix = Matrix::new(vec![
            vec![1.0, 5.0, 9.0],
            vec![-3.0, 2.0, 7.0],
            vec![0.0, 6.0, -3.0],
        ]);
        let sut = matrix.sub(0, 2);
        let expected = Matrix::new(vec![vec![-3.0, 2.0], vec![0.0, 6.0]]);
        assert_eq!(sut, expected);
    }
    #[test]
    fn sub_matrix_of_four_by_four_is_two_by_two() {
        let matrix = Matrix::new(vec![
            vec![-6.0, 1.0, 1.0, 6.0],
            vec![-8.0, 5.0, 8.0, 6.0],
            vec![-1.0, 0.0, 8.0, 2.0],
            vec![-7.0, 1.0, -1.0, 1.0],
        ]);
        let sut = matrix.sub(2, 1);
        let expected = Matrix::new(vec![
            vec![-6.0, 1.0, 6.0],
            vec![-8.0, 8.0, 6.0],
            vec![-7.0, -1.0, 1.0],
        ]);
        assert_eq!(sut, expected);
    }

    #[test]
    fn minor_of_matrix_is_correct() {
        let matrix = Matrix::new(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);
        let sub_determinant = matrix.sub(1, 0).determinant();
        let minor = matrix.minor(1, 0);
        assert_eq!(25.0, sub_determinant);
        assert_eq!(25.0, minor);
    }

    #[test]
    fn cofactor_of_three_by_three_matrix_is_correct() {
        let matrix = Matrix::new(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);
        assert_eq!(matrix.minor(0, 0), -12.0);
        assert_eq!(matrix.cofactor(0, 0), -12.0);
        assert_eq!(matrix.minor(1, 0), 25.0);
        assert_eq!(matrix.cofactor(1, 0), -25.0);
    }
    #[test]
    fn determinant_of_three_by_three_matrix_is_correct() {
        let matrix = Matrix::new(vec![
            vec![1.0, 2.0, 6.0],
            vec![-5.0, 8.0, -4.0],
            vec![2.0, 6.0, 4.0],
        ]);
        assert_eq!(matrix.cofactor(0, 0), 56.0);
        assert_eq!(matrix.cofactor(0, 1), 12.0);
        assert_eq!(matrix.cofactor(0, 2), -46.0);
        assert_eq!(matrix.determinant(), -196.0);
    }

    #[test]
    fn determinant_of_four_by_four_matrix_is_correct() {
        let matrix = Matrix::new(vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_eq!(matrix.cofactor(0, 0), 690.0);
        assert_eq!(matrix.cofactor(0, 1), 447.0);
        assert_eq!(matrix.cofactor(0, 2), 210.0);
        assert_eq!(matrix.cofactor(0, 3), 51.0);
        assert_eq!(matrix.determinant(), -4071.0);
    }

    #[test]
    fn invertible_matrix_is_invertible() {
        let matrix = Matrix::new(vec![
            vec![6.0, 4.0, 4.0, 4.0],
            vec![5.0, 5.0, 7.0, 6.0],
            vec![4.0, -9.0, 3.0, -8.0],
            vec![9.0, 1.0, 7.0, -6.0],
        ]);
        assert_ne!(matrix.determinant(), 0.0);
        assert!(matrix.inverse().is_some());
    }

    #[test]
    fn non_invertible_matrix_is_not_invertible() {
        let matrix = Matrix::new(vec![
            vec![-4.0, 2.0, -2.0, -3.0],
            vec![9.0, 6.0, 2.0, 6.0],
            vec![0.0, -5.0, 1.0, -5.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(matrix.determinant(), 0.0);
        assert!(matrix.inverse().is_none());
    }

    #[test]
    fn inverse_of_matrix_is_correct() {
        let matrix: Matrix = Matrix::new(vec![
            vec![-5.0, 2.0, 6.0, -8.0],
            vec![1.0, -5.0, 1.0, 8.0],
            vec![7.0, 7.0, -6.0, -7.0],
            vec![1.0, -3.0, 7.0, 4.0],
        ]);
        let expected: Matrix = Matrix::new(vec![
            vec![0.21805, 0.45113, 0.24060, -0.04511],
            vec![-0.80827, -1.45677, -0.44361, 0.52068],
            vec![-0.07895, -0.22368, -0.05263, 0.19737],
            vec![-0.52256, -0.81391, -0.30075, 0.30639],
        ]);
        let sut: Matrix = matrix.inverse().unwrap();
        assert_eq!(matrix.determinant(), 532.0);
        assert_eq!(matrix.cofactor(2, 3), -160.0);
        assert_eq!(sut.get(3, 2), -160.0 / 532.0);
        assert_eq!(matrix.cofactor(3, 2), 105.0);
        assert_eq!(sut.get(2, 3), 105.0 / 532.0);

        sut.matrix.into_iter().enumerate().for_each(|(i, row)| {
            row.into_iter().enumerate().for_each(|(j, col)| {
                let sut = col;
                let expected = expected.get(i, j);
                assert!(approx_eq!(f64, sut, expected, (0.00001, 1)));
            })
        })
    }

    #[test]
    fn multiplying_point_by_translation_matrix_produces_new_point() {
        let inverse = Matrix::translation(5.0, -3.0, 2.0).inverse().unwrap();
        let p1 = point(-3.0, 4.0, 5.0);
        let sut = inverse.mul_tup(p1);
        assert_eq!(point(-8.0, 7.0, 3.0), sut)
    }

    #[test]
    fn multiplying_point_by_inverse_matrix_produces_reverse() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let p1 = point(-3.0, 4.0, 5.0);
        let sut = transform.mul_tup(p1);
        assert_eq!(point(2.0, 1.0, 7.0), sut)
    }

    #[test]
    fn multiplying_vector_by_translation_is_ignored() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let v1 = vector(-3.0, 4.0, 5.0);
        let sut = transform.mul_tup(v1);
        assert_eq!((-3.0, 4.0, 5.0, 0.0), sut)
    }

    #[test]
    fn scaling_matrix_can_be_applied_to_point() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let p1 = point(-4.0, 6.0, 8.0);
        let sut = transform.mul_tup(p1);
        assert_eq!(sut, point(-8.0, 18.0, 32.0))
    }

    #[test]
    fn scaling_matrix_can_be_applied_to_vector() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let v1 = vector(-4.0, 6.0, 8.0);
        let sut = transform.mul_tup(v1);
        assert_eq!(sut, vector(-8.0, 18.0, 32.0))
    }

    #[test]
    fn inverse_of_scaling_matrix_shrinks_vector() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0).inverse().unwrap();
        let v1 = vector(-4.0, 6.0, 8.0);
        let sut = transform.mul_tup(v1);
        assert_eq!(sut, vector(-2.0, 2.0, 2.0))
    }

    #[test]
    fn scaling_can_reflect_vector() {
        let transform = Matrix::scaling(-1.0, 1.0, 1.0);
        let v1 = vector(2.0, 3.0, 4.0);
        let sut = transform.mul_tup(v1);
        assert_eq!(sut, vector(-2.0, 3.0, 4.0))
    }

    #[test]
    fn point_can_rotate_around_x_axis() {
        let p1: (f64, f64, f64, f64) = point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation(Axis::X, PI / 4.0);
        let full_quarter = Matrix::rotation(Axis::X, PI / 2.0);
        let sut_half = half_quarter.mul_tup(p1);
        let sut_full = full_quarter.mul_tup(p1);

        sut_half.approx_eq(point(0.0, 2.0.sqrt() / 2.0, 2.0.sqrt() / 2.0));
        sut_full.approx_eq(point(0.0, 0.0, 1.0));
    }

    #[test]
    fn rotation_is_reversed_with_inverse_of_matrix() {
        let p1 = point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation(Axis::X, PI / 4.0).inverse().unwrap();
        let sut_half = half_quarter.mul_tup(p1);
        sut_half.approx_eq(point(0.0, 2.0.sqrt() / 2.0, -(2.0.sqrt() / 2.0)));
    }

    #[test]
    fn point_can_rotate_around_y_axis() {
        let p1: (f64, f64, f64, f64) = point(0.0, 0.0, 1.0);
        let half_quarter = Matrix::rotation(Axis::Y, PI / 4.0);
        let full_quarter = Matrix::rotation(Axis::Y, PI / 2.0);
        let sut_half = half_quarter.mul_tup(p1);
        let sut_full = full_quarter.mul_tup(p1);
        sut_half.approx_eq(point(2.0.sqrt() / 2.0, 0.0, 2.0.sqrt() / 2.0));
        sut_full.approx_eq(point(1.0, 0.0, 0.0));
    }

    #[test]
    fn point_can_rotate_around_z_axis() {
        let p1: (f64, f64, f64, f64) = point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation(Axis::Z, PI / 4.0);
        let full_quarter = Matrix::rotation(Axis::Z, PI / 2.0);
        let sut_half = half_quarter.mul_tup(p1);
        let sut_full = full_quarter.mul_tup(p1);

        sut_half.approx_eq(point(-(2.0.sqrt() / 2.0), 2.0.sqrt() / 2.0, 0.0));
        sut_full.approx_eq(point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_y() {
        let p1: (f64, f64, f64, f64) = point(2.0, 3.0, 4.0);
        let shearing_trans = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let sut = shearing_trans.mul_tup(p1);
        let expected = point(5.0, 3.0, 4.0);
        assert_eq!(sut, expected);
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_z() {
        let p1: (f64, f64, f64, f64) = point(2.0, 3.0, 4.0);
        let shearing_trans = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let sut = shearing_trans.mul_tup(p1);
        let expected = point(6.0, 3.0, 4.0);
        assert_eq!(sut, expected);
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_x() {
        let p1: (f64, f64, f64, f64) = point(2.0, 3.0, 4.0);
        let shearing_trans = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let sut = shearing_trans.mul_tup(p1);
        let expected = point(2.0, 5.0, 4.0);
        assert_eq!(sut, expected);
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_z() {
        let p1: (f64, f64, f64, f64) = point(2.0, 3.0, 4.0);
        let shearing_trans = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let sut = shearing_trans.mul_tup(p1);
        let expected = point(2.0, 7.0, 4.0);
        assert_eq!(sut, expected);
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_x() {
        let p1: (f64, f64, f64, f64) = point(2.0, 3.0, 4.0);
        let shearing_trans = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let sut = shearing_trans.mul_tup(p1);
        let expected = point(2.0, 3.0, 6.0);
        assert_eq!(sut, expected);
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_y() {
        let p1: (f64, f64, f64, f64) = point(2.0, 3.0, 4.0);
        let shearing_trans = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let sut = shearing_trans.mul_tup(p1);
        let expected = point(2.0, 3.0, 7.0);
        assert_eq!(sut, expected);
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p1 = point(1.0, 0.0, 1.0);
        let a = Matrix::rotation(Axis::X, PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);

        let p2 = a.mul_tup(p1);
        p2.approx_eq(point(1.0, -1.0, 0.0));

        let p3 = b.mul_tup(p2);
        p3.approx_eq(point(5.0, -5.0, 0.0));

        let p4 = c.mul_tup(p3);
        p4.approx_eq(point(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations_are_applied_in_reverse_order() {
        let p1 = point(1.0, 0.0, 1.0);
        let a = Matrix::rotation(Axis::X, PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);
        let expected = c.mul_tup(b.mul_tup(a.mul_tup(p1)));
        expected.approx_eq(point(15.0, 0.0, 7.0));
    }

    #[test]
    fn fluid_interface_can_be_used_to_create_transform() {
        let p1 = point(1.0, 0.0, 1.0);
        let transform = Matrix::ident()
            .rotate(Axis::X, PI / 2.0)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);
        let expected = transform.mul_tup(p1);
        expected.approx_eq(point(15.0, 0.0, 7.0));
    }

    #[test]
    fn transform_matrix_for_default_orientation_is_ident() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, -1.0);
        let up = vector(0.0, 1.0, 0.0);
        let sut = Matrix::view_transform(from, to, up);
        assert_eq!(sut, Matrix::ident());
    }
    #[test]
    fn view_transform_matrix_looking_in_positive_z_direction() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, 1.0);
        let up = vector(0.0, 1.0, 0.0);
        let sut = Matrix::view_transform(from, to, up);
        assert_eq!(sut, Matrix::scaling(-1.0, 1.0, -1.0));
    }
    #[test]
    fn view_transform_moves_the_world() {
        let from = point(0.0, 0.0, 8.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        let sut = Matrix::view_transform(from, to, up);
        assert_eq!(sut, Matrix::translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn arbitrary_view_transform_is_correct() {
        let from = point(1.0, 3.0, 2.0);
        let to = point(4.0, -2.0, 8.0);
        let up = vector(1.0, 1.0, 0.0);
        let sut = Matrix::view_transform(from, to, up);
        let matrix = Matrix::new(vec![
            vec![-0.50709, 0.50709, 0.67612, -2.36643],
            vec![0.76772, 0.60609, 0.12122, -2.82843],
            vec![-0.35857, 0.59761, -0.71714, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);

       sut.approx_eq(matrix);
    }
}
