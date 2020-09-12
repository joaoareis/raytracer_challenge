use crate::point_vector::{PointVector,point,vector};
use crate::ray::Ray;
use crate::utils::compare_float;


pub trait Shape{}


#[derive(Debug)]
pub struct Sphere {
    pub center: PointVector,
    radius: f32 
}

impl Sphere {

    pub fn new() -> Sphere {
        Sphere{
            center: point(0,0,0),
            radius: 1.0
        }
    }

}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        compare_float(&self.radius, &other.radius) && (self.center == other.center)
        
    }
}
impl Eq for Sphere {}

impl Shape for Sphere {}