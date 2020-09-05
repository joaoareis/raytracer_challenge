use crate::point_vector::{PointVector,point,vector};

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
}


#[cfg(test)]
mod tests_ray {
    use super::*;

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
}