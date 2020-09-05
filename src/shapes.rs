use crate::point_vector::{PointVector,point,vector};
use crate::ray::Ray;

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