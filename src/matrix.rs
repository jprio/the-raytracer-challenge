use std::ops::Mul;

use crate::tuples::Tuple;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<const N: usize> {
    pub data: [[f64; N]; N],
}

impl<const N: usize> Mul<Matrix<N>> for Matrix<N> {
    type Output = Matrix<N>;
    fn mul(self, rhs: Matrix<N>) -> Self::Output {
        let mut matrix = Matrix::new();

        for row in 0..N {
            for column in 0..N {
                for i in 0..N {
                    matrix.data[row][column] += self.data[row][i] * rhs.data[i][column];
                }
            }
        }
        matrix
    }
}

impl Mul<Tuple> for Matrix<4> {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Self::Output {
        Tuple::new(
            self.data[0][0] * other.x
                + self.data[0][1] * other.y
                + self.data[0][2] * other.z
                + self.data[0][3] * other.w,
            self.data[1][0] * other.x
                + self.data[1][1] * other.y
                + self.data[1][2] * other.z
                + self.data[1][3] * other.w,
            self.data[2][0] * other.x
                + self.data[2][1] * other.y
                + self.data[2][2] * other.z
                + self.data[2][3] * other.w,
            self.data[3][0] * other.x
                + self.data[3][1] * other.y
                + self.data[3][2] * other.z
                + self.data[3][3] * other.w,
        )
    }
}

impl<const N: usize> Matrix<N> {
    fn new() -> Matrix<N> {
        return Matrix { data: [[0.; N]; N] };
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn can_instantiate_matrix() {
        let identity44 = Matrix {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        let identity22 = Matrix {
            data: [[1.0, 0.0], [0.0, 1.0]],
        };
        let identity22_2 = Matrix {
            data: [[1.0, 0.0], [0.0, 1.0]],
        };
        let mut zero22 = Matrix {
            data: [[0.0, 0.0], [0.0, 0.0]],
        };
        assert_eq!(identity22, identity22_2);
        assert_ne!(identity22, zero22);
        assert_eq!(zero22, zero22 * identity22);
    }
    #[test]
    fn can_mul_matrix() {
        let m1 = Matrix {
            data: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ],
        };
        let m2 = Matrix {
            data: [
                [-2.0, 1.0, 2.0, 3.0],
                [3.0, 2.0, 1.0, -1.0],
                [4.0, 3.0, 6.0, 5.0],
                [1.0, 2.0, 7.0, 8.0],
            ],
        };
        let m1_times_m2 = Matrix {
            data: [
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0],
            ],
        };
        assert_eq!(m1_times_m2, m1 * m2);
    }
}
