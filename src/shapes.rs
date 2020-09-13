use crate::point_vector::{PointVector,point,vector};
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::utils::compare_float;


pub trait Shape{}


#[derive(Debug)]
pub struct Sphere {
    pub center: PointVector,
    pub transform: Matrix,
    radius: f32 
}

impl Sphere {

    pub fn new() -> Sphere {
        Sphere{
            center: point(0,0,0),
            transform: Matrix::identity(4),
            radius: 1.0
        }
    }

    pub fn set_transform(&mut self, m: &Matrix) {
        self.transform = m.clone()
    }

}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        compare_float(&self.radius, &other.radius) && (self.center == other.center)
        
    }
}
impl Eq for Sphere {}

impl Shape for Sphere {}


#[cfg(test)]
mod tests_sphere {
    use super::*;
    use crate::transformations;

    #[test]
    fn test_transformation() {
        let s = Sphere::new();
        assert_eq!(s.transform, Matrix::identity(4));

    }

    #[test]
    fn test_transformation2() {
        let mut s = Sphere::new();
        let t = transformations::translate(2, 3, 4);
        s.set_transform(&t);
        assert_eq!(s.transform, t)

    }
}