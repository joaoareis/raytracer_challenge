use crate::point_vector::{PointVector,point,vector};
use crate::matrix::Matrix;

pub struct Ray {
    pub origin: PointVector,
    pub direction: PointVector
}

impl Ray {
    pub fn new(origin: PointVector, direction: PointVector) -> Ray {
        if !origin.is_point() {panic!("Origin must be a point.")}
        if !direction.is_vector() {panic!("Direction must be a vector.")}
        Ray {
            origin,
            direction
        }
    }

    pub fn position(&self, t: impl Into<f64>) -> PointVector {
        let t = t.into() as f32;
        self.origin + self.direction*t
    }

    pub fn transform(&self, transformation_matrix: &Matrix) -> Ray {
        let inv_transform = transformation_matrix.inverse();
        let new_origin = &inv_transform * &self.origin;
        let new_direction = &inv_transform * &self.direction;
        Ray::new(new_origin, new_direction)
    }
}


#[cfg(test)]
mod tests_ray {
    use super::*;
    use crate::transformations;

    #[test]
    fn test_new() {
        let p = point(1, 2, 3);
        let v = vector(4, 5, 6);
        let r = Ray::new(p,v);
        assert_eq!(r.origin, point(1, 2, 3));
        assert_eq!(r.direction, vector(4, 5, 6));
    }

    #[test]
    fn test_position() {
        let ray = Ray::new(point(2, 3, 4), vector(1, 0, 0));
        assert_eq!(ray.position(0), point(2, 3, 4));
        assert_eq!(ray.position(1), point(3, 3, 4));
        assert_eq!(ray.position(-1), point(1, 3, 4));
        assert_eq!(ray.position(2.5), point(4.5, 3.0, 4));
    }

    #[test]
    fn test_transform1() {
        let r = Ray::new(point(1, 2, 3), vector(0, 1, 0));
        let m = transformations::translate(3, 4, 5);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, point(4, 6, 8));
        assert_eq!(r2.direction, vector(0, 1, 0)); 
    }

    #[test]
    fn test_transform2() {
        let r = Ray::new(point(1, 2, 3), vector(0, 1, 0));
        let m = transformations::scaling(2, 3, 4);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, point(2, 6, 12));
        assert_eq!(r2.direction, vector(0, 3, 0)); 
    }
}