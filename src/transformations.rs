use crate::matrix::Matrix;
use crate::point_vector::{point,vector};


fn translate(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Matrix {
    let mut m = Matrix::identity(4);
    let x = x.into() as f32;
    let y = y.into() as f32;
    let z = z.into() as f32;
    m.set(0, 3, x);
    m.set(1, 3, y);
    m.set(2, 3, z);
    m
}


#[cfg(test)]
mod tests_matrix {
    use super::*;

    #[test]
    fn test_transform1() {
        let transform = translate(5,-3,2);
        let p = point(-3,4,5);
        assert_eq!(&transform*&p, point(2, 1, 7))
    }

    #[test]
    fn test_transform2() {
        let transform = translate(5,-3,2);
        let p = point(-3,4,5);
        assert_eq!(&transform.inverse()*&p, point(-8, 7, 3))
    }

    #[test]
    fn test_transform3() {
        let transform = translate(5,-3,2);
        let v = vector(-3,4,5);
        assert_eq!(&transform*&v, v)
    }
}


