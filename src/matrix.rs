use std::ops::{Add,Sub,Neg,Mul,Div};
use crate::utils::{compare_float};
use crate::point_vector::PointVector;



#[derive(Debug, Clone)]
struct Matrix {
    values : Vec<Vec<f32>>,
    i_dim: usize,
    j_dim: usize
}
fn mul_matrix_pointvector(m: &Matrix, p: &PointVector) -> PointVector {
    let matrix_values = vec![vec![p.x], vec![p.y], vec![p.z], vec![p.w]];
    let other_matrix = Matrix::new(matrix_values);
    let prod_matrix = m * &other_matrix;
    PointVector::new(prod_matrix.get(0,0), prod_matrix.get(1,0), prod_matrix.get(2,0), prod_matrix.get(3,0))
}

impl Matrix {
    fn new(values: Vec<Vec<f32>>) -> Matrix {
        let i_dim = values.len();
        let mut ctrl = true;
        let mut j_dim = 0;
        for row in values.iter() {
            if ctrl == true {
                j_dim = row.len();
                ctrl = false;
            } else {
                if row.len() != j_dim { println!("warning")};
            }
        }
        Matrix {
            values,
            i_dim,
            j_dim
        }
    }

    fn is_invertible(&self) -> bool {
        if self.determinant() == 0.0 { false}
        else { true}
    }
    fn submatrix(&self, row_to_remove: usize, col_to_remove: usize) -> Matrix{
        let mut submatrix_values: Vec<Vec<f32>> = vec![];
        for i in 0..self.shape().0 {
            if i != row_to_remove {
                let mut row: Vec<f32> = vec![];
                for j in 0..self.shape().1 {
                    if j!= col_to_remove {
                        row.push(self.get(i, j));
                    }
                }
                submatrix_values.push(row);
            }
        }
        Matrix::new(submatrix_values)
    }

    fn shape(&self) -> (usize, usize) {
        (self.i_dim, self.j_dim)
    }

    fn get(&self, i: usize, j: usize) -> f32 {
        self.values[i][j]
    }

    fn determinant_2d(&self) -> f32 {
        if self.shape() != (2,2) {
            panic!("Invalid shape.")
        }
        self.get(0,0)*self.get(1,1) - self.get(1,0)*self.get(0,1)
    }
    fn determinant(&self) -> f32 {
        if self.shape() == (2,2) {
            self.determinant_2d()
        }
        else {
            self.determinant_higher_d()
        }
    }

    fn determinant_higher_d(&self) -> f32 {
        let mut determinant: f32 = 0.0;
        for (col,elem) in self.values[0].iter().enumerate() {
            determinant = determinant + elem*self.cofactor(0, col);
        }
        determinant
    }

    fn minor(&self, row: usize, col: usize) -> f32 {
        let submatrix = self.submatrix(row, col);
        submatrix.determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> f32 {
        let submatrix = self.submatrix(row, col);
        if (row+col)%2 == 0 { submatrix.determinant()}
        else {-submatrix.determinant()}
    }

    fn transpose(&self) -> Matrix {
        let mut values: Vec<Vec<f32>> = vec![];
        for i in 0..self.shape().0 {
            let mut row: Vec<f32> = vec![];
            for j in 0..self.shape().1 {
                row.push(self.get(j, i));
            }
            values.push(row);
        }
        Matrix::new(values)

    }

    fn identity(diag_size: usize) -> Matrix {
        let mut values: Vec<Vec<f32>> = vec![];
        for i in 0..diag_size {
            let mut row: Vec<f32> = vec![];
            for j in 0..diag_size {
                if i == j { row.push(1.0)}
                else {row.push(0.0)}
            }
            values.push(row);
        }
        Matrix::new(values)
    }

    fn multiply(&self, other: &Matrix) -> Matrix {
        let other_shape = other.shape();
        let this_shape = self.shape();
        if this_shape.1 != other_shape.0 {
            panic!("Shapes are not valid.")
        }
        let nr_rows_new_matrix = this_shape.0;
        let nr_columns_new_matrix = other_shape.1;
        let inner_dimension = this_shape.1;
        let mut values: Vec<Vec<f32>> = vec![];
        for i in 0..nr_rows_new_matrix {
            let mut row: Vec<f32> = vec![];
            for j in 0..nr_columns_new_matrix {
                let mut value : f32 = 0.0;
                for m in 0..inner_dimension {
                    value = value + self.get(i,m)*other.get(m,j);
                }
                row.push(value);
            }
            values.push(row);
        }
        Matrix::new(values)

    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.shape() != other.shape() {
            false
        } else {
            for (i, row) in self.values.iter().enumerate() {
                for (j, value) in row.iter().enumerate() {
                    if !compare_float(&value, &other.get(i,j)) {
                        return false
                    }
                }
            }
            true
        }
    }
}
impl Eq for Matrix {}

impl Mul<&Matrix> for &Matrix {

    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        self.multiply(&rhs)
    }
}

impl Mul<&PointVector> for &Matrix {

    type Output = PointVector;

    fn mul(self, rhs: &PointVector) -> Self::Output {
        mul_matrix_pointvector(self, rhs)
    }
}

#[cfg(test)]
mod tests_matrix {
    use super::*;

    #[test]
    fn test_new() {
        let values = vec![vec![1.0,2.0,3.0,4.0],vec![5.5, 6.5, 7.5, 8.5], vec![9.0, 10.0, 11.0, 12.0], vec![13.5, 14.5, 15.5, 16.5]];
        let m = Matrix::new(values);
        assert_eq!(m.get(0,0), 1.0);
        assert_eq!(m.get(0,3), 4.0);
        assert_eq!(m.get(1,0), 5.5);
        assert_eq!(m.get(1,2), 7.5);
        assert_eq!(m.get(2,2), 11.0);
        assert_eq!(m.get(3,0), 13.5);
        assert_eq!(m.get(3,2), 15.5);
    }
    #[test]
    fn test_new2() {
        let values = vec![vec![-3.0,5.0],vec![1.0,-2.0]];
        let m = Matrix::new(values);
        assert_eq!(m.get(0,0), -3.0);
        assert_eq!(m.get(0,1), 5.0);
        assert_eq!(m.get(1,0), 1.0);
        assert_eq!(m.get(1,1), -2.0);

    }
    #[test]
    fn test_new3() {
        let values = vec![vec![-3.0,5.0,0.0],vec![1.0,-2.0,-7.0], vec![0.0,1.0,1.0]];
        let m = Matrix::new(values);
        assert_eq!(m.get(0,0), -3.0);
        assert_eq!(m.get(1,1), -2.0);
        assert_eq!(m.get(2,2), 1.0);

    }

    #[test]
    fn test_eq() {
        let values = vec![vec![-3.0,5.0],vec![1.0,-2.0]];
        let m = Matrix::new(values);
        let values_n = vec![vec![-3.0,5.0],vec![1.0,-2.0]];
        let n = Matrix::new(values_n);
        assert_eq!(m, n);
    }

    #[test]
    fn test_eq2() {
        let values = vec![vec![1.0,2.0,3.0,4.0],vec![5.0,6.0,7.0,8.0],vec![9.0,8.0,7.0,6.0],vec![5.0,4.0,3.0,2.0]];
        let m = Matrix::new(values);
        let values_n = vec![vec![1.0,2.0,3.0,4.0],vec![5.0,6.0,7.0,8.0],vec![9.0,8.0,7.0,6.0],vec![5.0,4.0,3.0,2.0]];
        let n = Matrix::new(values_n);
        assert_eq!(m, n);
    }

    #[test]
    fn test_eq3() {
        let values = vec![vec![1.0,2.0,3.0,4.0],vec![5.0,6.0,7.0,8.0],vec![9.0,8.0,7.0,6.0],vec![5.0,4.0,3.0,2.0]];
        let m = Matrix::new(values);
        let values_n = vec![vec![2.0,3.0,4.0,5.0],vec![6.0,7.0,8.0,9.0],vec![8.0,7.0,6.0,5.0],vec![4.0,3.0,2.0,1.0]];
        let n = Matrix::new(values_n);
        assert_ne!(m, n);
    }

    #[test]
    fn test_mul() {
        let values = vec![vec![1.0,2.0,3.0,4.0],vec![5.0,6.0,7.0,8.0],vec![9.0,8.0,7.0,6.0],vec![5.0,4.0,3.0,2.0]];
        let m = Matrix::new(values);
        let values_n = vec![vec![-2.0,1.0,2.0,3.0],vec![3.0,2.0,1.0,-1.0],vec![4.0,3.0,6.0,5.0],vec![1.0,2.0,7.0,8.0]];
        let n = Matrix::new(values_n);

        let expected = Matrix::new(vec![vec![20.0,22.0,50.0,48.0],vec![44.0,54.0,114.0,108.0],vec![40.0,58.0,110.0,102.0],vec![16.0,26.0,46.0,42.0]]);
        assert_eq!(&m*&n, expected);
    }

    #[test]
    fn test_mul_pv() {
        let values = vec![vec![1.0,2.0,3.0,4.0],vec![2.0,4.0,4.0,2.0],vec![8.0, 6.0, 4.0, 1.0],vec![0.0,0.0,0.0,1.0]];
        let m = Matrix::new(values);
        let p = PointVector::new(1,2,3,1);
        let res = &m * &p;
        assert_eq!(res, PointVector::new(18,24,33,1));
    }

    #[test]
    fn test_mul_identity() {
        let id = Matrix::identity(4);
        let values = vec![vec![0.0,1.0,2.0,4.0],vec![1.0,2.0,4.0,8.0],vec![2.0, 4.0, 8.0, 16.0],vec![4.0,8.0,16.0,32.0]];
        let m = Matrix::new(values);
        let p = PointVector::new(1,2,3,1);

        assert_eq!(&id*&m, m);
        assert_eq!(&id*&p, p);


    }

    #[test]
    fn test_transpose(){
        let id = Matrix::identity(4);
        let values = vec![vec![0.0,9.0,3.0,0.0],vec![9.0,8.0,0.0,8.0],vec![1.0, 8.0, 5.0, 3.0],vec![0.0,0.0,5.0,8.0]];
        let m = Matrix::new(values);
        let m_transposed = Matrix::new(vec![vec![0.0,9.0,1.0,0.0],vec![9.0,8.0,8.0,0.0],vec![3.0, 0.0, 5.0, 5.0],vec![0.0,8.0,3.0,8.0]]);
        
        assert_eq!(m.transpose(), m_transposed);
        assert_eq!(id.transpose(), id);

    }

    #[test]
    fn test_determinant() {
        let m = Matrix::new(vec![vec![1.0,5.0], vec![-3.0,2.0]]);
        assert_eq!(m.determinant(),17.0)
    }

    #[test]
    fn test_submatrix() {
        let m1 = Matrix::new(vec![vec![1.0,5.0,0.0],vec![-3.0,2.0,7.0],vec![0.0,6.0,-3.0]]);
        assert_eq!(m1.submatrix(0, 2), Matrix::new(vec![vec![-3.0,2.0],vec![0.0,6.0]]));

        let m2 = Matrix::new(vec![vec![-6.0,1.0,1.0,6.0],vec![-8.0,5.0,8.0,6.0],vec![-1.0,0.0,8.0,2.0],vec![-7.0,1.0,-1.0,1.0]]);
        assert_eq!(m2.submatrix(2, 1), Matrix::new(vec![vec![-6.0,1.0, 6.0],vec![-8.0,8.0,6.0],vec![-7.0,-1.0,1.0]]));
        
    }

    #[test]
    fn test_minor() {
        let m1 = Matrix::new(vec![vec![3.0,5.0,0.0],vec![2.0,-1.0,-7.0],vec![6.0,-1.0,5.0]]);
        assert_eq!(m1.minor(1,0),25.0)
    }

    #[test]
    fn test_cofactor() {
        let m1 = Matrix::new(vec![vec![3.0,5.0,0.0],vec![2.0,-1.0,-7.0],vec![6.0,-1.0,5.0]]);
        assert_eq!(m1.cofactor(0,0),-12.0);
        assert_eq!(m1.minor(0,0),-12.0);
        assert_eq!(m1.minor(1,0),25.0);
        assert_eq!(m1.cofactor(1,0),-25.0);
    }

    #[test]
    fn test_3d_4d_determinant() {
        let m1 = Matrix::new(vec![vec![1.0,2.0,6.0],vec![-5.0,8.0,-4.0],vec![2.0,6.0,4.0]]);
        assert_eq!(m1.cofactor(0,0),56.0);
        assert_eq!(m1.cofactor(0,1),12.0);
        assert_eq!(m1.cofactor(0,2),-46.0);
        assert_eq!(m1.determinant(),-196.0);

        let m2 = Matrix::new(vec![vec![-2.0,-8.0,3.0,5.0],vec![-3.0,1.0,7.0,3.0],vec![1.0,2.0,-9.0,6.0],vec![-6.0,7.0,7.0,-9.0]]);
        assert_eq!(m2.cofactor(0,0),690.0);
        assert_eq!(m2.cofactor(0,1),447.0);
        assert_eq!(m2.cofactor(0,2),210.0);
        assert_eq!(m2.cofactor(0,3),51.0);
        assert_eq!(m2.determinant(),-4071.0);
    }
}
