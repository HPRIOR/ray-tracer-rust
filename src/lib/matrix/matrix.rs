use std::ops::Mul;

use num_traits::{Num, Zero};

type MatrixVec<T> = Vec<Vec<T>>;

#[derive(PartialEq, Eq, Debug)]
pub struct Matrix<T> {
    matrix: MatrixVec<T>,
}

impl<T: Clone + Copy> Matrix<T> {
    fn new(matrix: MatrixVec<T>) -> Self {
        Self { matrix }
    }

    fn get(&self, row: usize, col: usize) -> T {
        self.matrix[row][col]
    }
}

impl<T> Mul for Matrix<T>
where
    T: Num + Copy + Clone,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let length = self.matrix.len();
        let width = rhs.matrix[0].len();

        Matrix::new(
            (0..length)
                .map(|i| {
                    (0..width)
                        .map(move |j| (i, j))
                        .map(|(i, j)| {
                            (0..length).fold(Zero::zero(), |acc, x| {
                                acc + self.matrix[i][x] * rhs.matrix[x][j]
                            })
                        })
                        .collect()
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
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
        let sut = m1 * m2;
        let expected: Matrix<i8> = Matrix::new(vec![
            vec![20, 22, 50, 48],
            vec![44, 54, 114, 108],
            vec![40, 58, 110, 102],
            vec![16, 26, 46, 42],
        ]);
        assert_eq!(sut, expected);
    }

    #[test]
    fn matrix_can_be_multiplied_by_tuple() {}
}
