use std::fmt::Display;

use num_traits::{Num, One, Zero};

type MatrixVec<T> = Vec<Vec<T>>;

#[derive(PartialEq, Eq, Debug)]
pub struct Matrix<T> {
    matrix: MatrixVec<T>,
}

trait MultiplyByMatrix<T: Copy + Clone + Num> {
    fn mul(&self, matrix: &Matrix<T>) -> Matrix<T>;
}

impl<T: Copy + Clone + Num + Display> MultiplyByMatrix<T> for &Matrix<T> {
    fn mul(&self, rhs: &Matrix<T>) -> Matrix<T> {
        let length = self.matrix.len();
        let width = rhs.matrix[0].len();

        Matrix::new(
            (0..length)
                .map(|i| {
                    (0..width)
                        .map(move |j| (i, j))
                        .map(|(i, j)| {
                            (0..length).fold(Zero::zero(), |acc, x| {
                                acc + self.matrix[x][j] * rhs.matrix[i][x]
                            })
                        })
                        .collect()
                })
                .collect(),
        )
    }
}

impl<T: Copy + Clone + Num + Display> MultiplyByMatrix<T> for (T, T, T, T) {
    fn mul(&self, rhs: &Matrix<T>) -> Matrix<T> {
        fn muliply_row<T>(row: &Vec<T>, tuple: (T, T, T, T)) -> T
        where
            T: Num + Copy + Clone,
        {
            row[0] * tuple.0 + row[1] * tuple.1 + row[2] * tuple.2 + row[3] * tuple.3
        }

        Matrix::new(vec![vec![
            muliply_row(&rhs.matrix[0], *self),
            muliply_row(&rhs.matrix[1], *self),
            muliply_row(&rhs.matrix[2], *self),
            muliply_row(&rhs.matrix[3], *self),
        ]])
    }
}

impl<T: Num + Clone + Copy + Display> Matrix<T> {
    fn new(matrix: MatrixVec<T>) -> Self {
        Self { matrix }
    }

    fn ident() -> Self {
        Self {
            matrix: vec![
                vec![One::one(), Zero::zero(), Zero::zero(), Zero::zero()],
                vec![Zero::zero(), One::one(), Zero::zero(), Zero::zero()],
                vec![Zero::zero(), Zero::zero(), One::one(), Zero::zero()],
                vec![Zero::zero(), Zero::zero(), Zero::zero(), One::one()],
            ],
        }
    }

    fn as_tuple(&self) -> (T, T, T, T) {
        (
            self.matrix[0][0],
            self.matrix[0][1],
            self.matrix[0][2],
            self.matrix[0][3],
        )
    }

    fn get(&self, row: usize, col: usize) -> T {
        self.matrix[row][col]
    }

    fn mul<A: MultiplyByMatrix<T>>(&self, arg: A) -> Matrix<T> {
        arg.mul(self)
    }

    fn transpose(&self) -> Matrix<T> {
        let matrix = &self.matrix;

        let mut new_matrix = self.matrix.clone();

        for (i, row) in matrix.into_iter().enumerate() {
            for (j, col) in row.into_iter().enumerate() {
                new_matrix[j][i] = *col;
            }
        }

        return Matrix::new(new_matrix);
    }

    fn determinant(&self) -> T {
        let matrix = &self.matrix;
        if matrix.len() == 2 {
            return (matrix[0][0] * matrix[1][1]) - (matrix[0][1] * matrix[1][0]);
        }

        matrix[0]
            .iter()
            .enumerate()
            .fold(Zero::zero(), |acc, (i, x)| (acc + *x * self.cofactor(0, i)))
    }

    fn sub(&self, row_size: usize, col_size: usize) -> Matrix<T> {
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

    fn minor(&self, row_size: usize, col_size: usize) -> T {
        let sub_matrix = &self.sub(row_size, col_size);
        sub_matrix.determinant()
    }

    fn cofactor(&self, row_size: usize, col_size: usize) -> T {
        let minor = self.minor(row_size, col_size);
        if (row_size + col_size) % 2 == 0 {
            minor
        } else {
            let zero: T = Zero::zero();
            zero - minor
        }
    }

    fn inverse(&self) -> Option<Matrix<T>> {
        if self.determinant() == Zero::zero() {
            None
        } else {
            Some(Matrix::new(vec![vec![]]))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Sub;

    use super::Matrix;

    #[test]
    fn matrix_elements_are_correct() {
        let matrix: Matrix<f32> = Matrix::new(vec![
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
        let matrix: Matrix<f32> = Matrix::new(vec![vec![-3.0, 5.0], vec![1.0, -2.0]]);
        assert_eq!(matrix.get(0, 0), -3.0);
        assert_eq!(matrix.get(0, 1), 5.0);
        assert_eq!(matrix.get(1, 0), 1.0);
        assert_eq!(matrix.get(1, 1), -2.0);
    }

    #[test]
    fn can_represent_three_by_three_matrix() {
        let matrix: Matrix<i8> = Matrix::new(vec![vec![-3, 5, 0], vec![1, -2, -7], vec![0, 1, 1]]);
        assert_eq!(matrix.get(0, 0), -3);
        assert_eq!(matrix.get(0, 1), 5);
        assert_eq!(matrix.get(0, 2), 0);
        assert_eq!(matrix.get(1, 0), 1);
        assert_eq!(matrix.get(1, 1), -2);
        assert_eq!(matrix.get(1, 2), -7);
        assert_eq!(matrix.get(2, 0), 0);
        assert_eq!(matrix.get(2, 1), 1);
        assert_eq!(matrix.get(2, 2), 1);
    }

    #[test]
    fn matrix_are_equal() {
        let m1: Matrix<i8> = Matrix::new(vec![vec![-3, 5, 0], vec![1, -2, -7], vec![0, 1, 1]]);
        let m2: Matrix<i8> = Matrix::new(vec![vec![-3, 5, 0], vec![1, -2, -7], vec![0, 1, 1]]);
        let sut = m1 == m2;
        assert!(sut);
    }

    #[test]
    fn matrix_are_ne() {
        let m1: Matrix<i8> = Matrix::new(vec![vec![-3, 5, 0], vec![1, -2, -7], vec![0, 1, 1]]);
        let m2: Matrix<i8> = Matrix::new(vec![vec![-3, 6, 0], vec![2, -2, -7], vec![0, 1, 1]]);
        let sut = m1 != m2;
        assert!(sut);
    }

    #[test]
    fn matrix_can_be_multiplied_together() {
        let m1: Matrix<i8> = Matrix::new(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 8, 7, 6],
            vec![5, 4, 3, 2],
        ]);
        let m2: Matrix<i8> = Matrix::new(vec![
            vec![-2, 1, 2, 3],
            vec![3, 2, 1, -1],
            vec![4, 3, 6, 5],
            vec![1, 2, 7, 8],
        ]);
        let sut = m1.mul(&m2);
        let expected: Matrix<i8> = Matrix::new(vec![
            vec![20, 22, 50, 48],
            vec![44, 54, 114, 108],
            vec![40, 58, 110, 102],
            vec![16, 26, 46, 42],
        ]);
        assert_eq!(sut, expected);
    }

    #[test]
    fn matrix_can_be_multiplied_by_tuple() {
        let matrix: Matrix<i8> = Matrix::new(vec![
            vec![1, 2, 3, 4],
            vec![2, 4, 4, 2],
            vec![8, 6, 4, 1],
            vec![0, 0, 0, 1],
        ]);

        let tuple = (1, 2, 3, 1);

        let sut = matrix.mul(tuple);
        let expected = (18, 24, 33, 1);
        assert_eq!(sut.as_tuple(), expected);
    }

    #[test]
    fn matrix_multiplied_by_identity_produces_original() {
        let matrix: Matrix<i8> = Matrix::new(vec![
            vec![1, 2, 3, 4],
            vec![2, 4, 4, 2],
            vec![8, 6, 4, 1],
            vec![0, 0, 0, 1],
        ]);

        let sut = matrix.mul(&Matrix::ident());

        assert_eq!(sut, matrix);
    }

    #[test]
    fn matrix_transposes_correctly() {
        let matrix: Matrix<i8> = Matrix::new(vec![
            vec![0, 9, 3, 0],
            vec![9, 8, 0, 8],
            vec![1, 8, 5, 3],
            vec![0, 0, 5, 8],
        ]);

        let sut = matrix.transpose();
        let expected = Matrix::new(vec![
            vec![0, 9, 1, 0],
            vec![9, 8, 8, 0],
            vec![3, 0, 5, 5],
            vec![0, 8, 3, 8],
        ]);

        assert_eq!(sut, expected);
    }

    #[test]
    fn transpose_of_idendity_is_identidy() {
        let ident: Matrix<i8> = Matrix::ident();
        let sut = ident.transpose();
        assert_eq!(sut, ident)
    }

    #[test]
    fn determinant_base_is_correct() {
        let matrix = Matrix::new(vec![vec![1, 5], vec![-3, 2]]);
        let sut = matrix.determinant();
        assert_eq!(sut, 17);
    }

    #[test]
    fn sub_matrix_of_three_by_three_is_two_by_two() {
        let matrix = Matrix::new(vec![vec![1, 5, 9], vec![-3, 2, 7], vec![0, 6, -3]]);
        let sut = matrix.sub(0, 2);
        let expected = Matrix::new(vec![vec![-3, 2], vec![0, 6]]);
        assert_eq!(sut, expected);
    }
    #[test]
    fn sub_matrix_of_four_by_four_is_two_by_two() {
        let matrix = Matrix::new(vec![
            vec![-6, 1, 1, 6],
            vec![-8, 5, 8, 6],
            vec![-1, 0, 8, 2],
            vec![-7, 1, -1, 1],
        ]);
        let sut = matrix.sub(2, 1);
        let expected = Matrix::new(vec![vec![-6, 1, 6], vec![-8, 8, 6], vec![-7, -1, 1]]);
        assert_eq!(sut, expected);
    }

    #[test]
    fn minor_of_matrix_is_correct() {
        let matrix = Matrix::new(vec![vec![3, 5, 0], vec![2, -1, -7], vec![6, -1, 5]]);
        let sub_determinant = matrix.sub(1, 0).determinant();
        let minor = matrix.minor(1, 0);
        assert_eq!(25, sub_determinant);
        assert_eq!(25, minor);
    }

    #[test]
    fn cofactor_of_three_by_three_matrix_is_correct() {
        let matrix = Matrix::new(vec![vec![3, 5, 0], vec![2, -1, -7], vec![6, -1, 5]]);
        assert_eq!(matrix.minor(0, 0), -12);
        assert_eq!(matrix.cofactor(0, 0), -12);
        assert_eq!(matrix.minor(1, 0), 25);
        assert_eq!(matrix.cofactor(1, 0), -25);
    }
    #[test]
    fn determinant_of_three_by_three_matrix_is_correct() {
        let matrix = Matrix::new(vec![vec![1, 2, 6], vec![-5, 8, -4], vec![2, 6, 4]]);
        assert_eq!(matrix.cofactor(0, 0), 56);
        assert_eq!(matrix.cofactor(0, 1), 12);
        assert_eq!(matrix.cofactor(0, 2), -46);
        assert_eq!(matrix.determinant(), -196);
    }

    #[test]
    fn determinant_of_four_by_four_matrix_is_correct() {
        let matrix = Matrix::new(vec![
            vec![-2, -8, 3, 5],
            vec![-3, 1, 7, 3],
            vec![1, 2, -9, 6],
            vec![-6, 7, 7, -9],
        ]);
        assert_eq!(matrix.cofactor(0, 0), 690);
        assert_eq!(matrix.cofactor(0, 1), 447);
        assert_eq!(matrix.cofactor(0, 2), 210);
        assert_eq!(matrix.cofactor(0, 3), 51);
        assert_eq!(matrix.determinant(), -4071);
    }

    #[test]
    fn invertable_matrix_is_invertable() {
        let matrix = Matrix::new(vec![
            vec![6, 4, 4, 4],
            vec![5, 5, 7, 6],
            vec![4, -9, 3, -8],
            vec![9, 1, 7, -6],
        ]);
        assert!(matrix.inverse().is_some());
    }

    #[test]
    fn non_invertable_matrix_is_not_invertable() {
        let matrix = Matrix::new(vec![
            vec![-4, 2, -2, -3],
            vec![9, 6, 2, 6],
            vec![0, -5, 1, -5],
            vec![0, 0, 0, 0],
        ]);

        assert!(matrix.inverse().is_none());
    }
}
