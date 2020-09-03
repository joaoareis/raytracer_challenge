use crate::matrix::Matrix;
use crate::point_vector::{point,vector};
use std::f32::consts;


pub fn translate(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Matrix {
    let mut m = Matrix::identity(4);
    let x = x.into() as f32;
    let y = y.into() as f32;
    let z = z.into() as f32;
    m.set(0, 3, x);
    m.set(1, 3, y);
    m.set(2, 3, z);
    m
}

pub fn scaling(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Matrix {
    let mut m = Matrix::identity(4);
    let x = x.into() as f32;
    let y = y.into() as f32;
    let z = z.into() as f32;
    m.set(0, 0, x);
    m.set(1, 1, y);
    m.set(2, 2, z);
    m
}

pub fn rotation_x(r: impl Into<f64>) -> Matrix {
    let mut m = Matrix::identity(4);
    let r = r.into() as f32;
    let c = r.cos();
    let s = r.sin();
    m.set(1, 1, c);
    m.set(1, 2, -s);
    m.set(2, 1, s);
    m.set(2, 2, c);
    m
}

pub fn rotation_y(r: impl Into<f64>) -> Matrix {
    let mut m = Matrix::identity(4);
    let r = r.into() as f32;
    let c = r.cos();
    let s = r.sin();
    m.set(0, 0, c);
    m.set(2, 0, -s);
    m.set(0, 2, s);
    m.set(2, 2, c);
    m
}

pub fn rotation_z(r: impl Into<f64>) -> Matrix {
    let mut m = Matrix::identity(4);
    let r = r.into() as f32;
    let c = r.cos();
    let s = r.sin();
    m.set(0, 0, c);
    m.set(0, 1, -s);
    m.set(1, 0, s);
    m.set(1, 1, c);
    m
}

pub fn shearing(x_y: impl Into<f64>, x_z: impl Into<f64>, y_x: impl Into<f64>, y_z: impl Into<f64>, z_x: impl Into<f64>, z_y: impl Into<f64>) -> Matrix {
    let mut m = Matrix::identity(4);
    let x_y = x_y.into() as f32;
    let x_z = x_z.into() as f32;
    let y_x = y_x.into() as f32;
    let y_z = y_z.into() as f32;
    let z_x = z_x.into() as f32;
    let z_y = z_y.into() as f32;
    m.set(0, 1, x_y);
    m.set(0, 2, x_z);
    m.set(1, 0, y_x);
    m.set(1, 2, y_z);
    m.set(2, 0, z_x);
    m.set(2, 1, z_y);
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

    #[test]
    fn test_scaling() {
        let transform = scaling(2, 3, 4);
        let v = point(-4,6,8);
        assert_eq!(&transform*&v, point(-8, 18, 32))
    }
    #[test]
    fn test_scaling2() {
        let transform = scaling(2, 3, 4);
        println!("{:?}", transform);
        let v = vector(-4,6,8);
        assert_eq!(&transform*&v, vector(-8, 18, 32))
    }

    #[test]
    fn test_scaling3() {
        let transform = scaling(2, 3, 4).inverse();
        let v = vector(-4,6,8);
        assert_eq!(&transform*&v, vector(-2, 2, 2))
    }

    #[test]
    fn test_reflection() {
        let transform = scaling(-1, 1, 1);
        let v = point(2,3,4);
        assert_eq!(&transform*&v, point(-2, 3, 4))
    }

    #[test]
    fn test_rotation_x() {
        let v = point(0,1,0);
        let half_quarter = rotation_x(consts::PI/4.0);
        let full_quarter = rotation_x(consts::PI/2.0);
        assert_eq!(&half_quarter*&v, point(0, (2.0_f32).sqrt()/2.0, (2.0_f32).sqrt()/2.0));
        assert_eq!(&full_quarter*&v, point(0, 0, 1));
        assert_eq!(&half_quarter.inverse()*&v, point(0, (2.0_f32).sqrt()/2.0, -(2.0_f32).sqrt()/2.0));
        
    }

    #[test]
    fn test_rotation_y() {
        let v = point(0,0,1);
        let half_quarter = rotation_y(consts::PI/4.0);
        let full_quarter = rotation_y(consts::PI/2.0);
        assert_eq!(&half_quarter*&v, point((2.0_f32).sqrt()/2.0, 0, (2.0_f32).sqrt()/2.0));
        assert_eq!(&full_quarter*&v, point(1, 0, 0));
        
    }

    #[test]
    fn test_rotation_z() {
        let v = point(0,1,0);
        let half_quarter = rotation_z(consts::PI/4.0);
        let full_quarter = rotation_z(consts::PI/2.0);
        assert_eq!(&half_quarter*&v, point(-(2.0_f32).sqrt()/2.0, (2.0_f32).sqrt()/2.0, 0));
        assert_eq!(&full_quarter*&v, point(-1, 0, 0));
        
    }

    #[test]
    fn shearing1() {
        let transform = shearing(1,0,0,0,0,0);
        let p1 = point(2,3,4);
        assert_eq!(&transform * &p1, point(5, 3, 4))
        
    }

    #[test]
    fn shearing2() {
        let transform = shearing(0,1,0,0,0,0);
        let p1 = point(2,3,4);
        assert_eq!(&transform * &p1, point(6, 3, 4))
        
    }

    #[test]
    fn shearing3() {
        let transform = shearing(0,0,1,0,0,0);
        let p1 = point(2,3,4);
        assert_eq!(&transform * &p1, point(2, 5, 4))
        
    }

    #[test]
    fn shearing4() {
        let transform = shearing(0,0,0,1,0,0);
        let p1 = point(2,3,4);
        assert_eq!(&transform * &p1, point(2, 7, 4))
        
    }

    #[test]
    fn shearing5() {
        let transform = shearing(0,0,0,0,1,0);
        let p1 = point(2,3,4);
        assert_eq!(&transform * &p1, point(2, 3, 6))
        
    }

    #[test]
    fn shearing6() {
        let transform = shearing(0,0,0,0,0,1);
        let p1 = point(2,3,4);
        assert_eq!(&transform * &p1, point(2, 3, 7))
        
    }

    #[test]
    fn test_all_transformations1() {
        let p = point(1,0,1);
        let A = rotation_x(consts::PI/2.0);
        let B = scaling(5,5,5);
        let C = translate(10, 5, 7);

        let p2 = &A * &p;
        assert_eq!(p2, point(1,-1,0));

        let p3 = &B * &p2;
        assert_eq!(p3, point(5, -5, 0));

        let p4 = &C * &p3;
        assert_eq!(p4, point(15, 0, 7));



    }

    #[test]
    fn test_all_transformations2() {
        let p = point(1,0,1);
        let A = rotation_x(consts::PI/2.0);
        let B = scaling(5,5,5);
        let C = translate(10, 5, 7);

        assert_eq!(&(&(&C * &B) * &A) * &p, point(15, 0, 7));



    }

}