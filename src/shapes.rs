use crate::point_vector::{PointVector,point,vector};
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::material::Material;
use crate::utils::compare_float;


pub trait Shape{}


#[derive(Debug)]
pub struct Sphere {
    pub center: PointVector,
    pub transform: Matrix,
    pub material: Material,
    radius: f32 
}

impl Sphere {

    pub fn new() -> Sphere {
        Sphere{
            center: point(0,0,0),
            transform: Matrix::identity(4),
            material: Material::default(),
            radius: 1.0
        }
    }

    pub fn set_transform(&mut self, m: &Matrix) {
        self.transform = m.clone()
    }

    pub fn normal_at(&self, world_point: PointVector) -> PointVector {
        let object_point = &self.transform.inverse() * &world_point;
        let mut normal_vector = (object_point - point(0, 0, 0)).normalize();
        normal_vector = &self.transform.inverse().transpose() * &normal_vector;
        normal_vector.w = 0.0;
        normal_vector.normalize()
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
    #[test]
    fn test_normal_at() {
        let s = Sphere::new();
        let n = s.normal_at(point(1,0,0));
        assert_eq!(n, vector(1, 0, 0));
    }
    #[test]
    fn test_normal_at2() {
        let s = Sphere::new();
        let n = s.normal_at(point(0,1,0));
        assert_eq!(n, vector(0, 1, 0));
    }
    #[test]
    fn test_normal_at3() {
        let s = Sphere::new();
        let n = s.normal_at(point(0,0,1));
        assert_eq!(n, vector(0, 0, 1));
    }
    #[test]
    fn test_normal_at4() {
        let s = Sphere::new();
        let c = 3.0_f32.sqrt()/3.0;
        let n = s.normal_at(point(c,c,c));
        assert_eq!(n, vector(c, c, c));
    }

    #[test]
    fn test_normal_at5() {
        let s = Sphere::new();
        let c = 3.0_f32.sqrt()/3.0;
        let n = s.normal_at(point(c,c,c));
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn test_normal_at6() {
        let mut s = Sphere::new();
        s.set_transform(&transformations::translate(0, 1, 0));
        let n = s.normal_at(point(0,1.70711,-0.70711));
        assert_eq!(n, vector(0, 0.70711, -0.70711));
    }

    #[test]
    fn test_normal_at7() {
        let mut s = Sphere::new();
        let m = &transformations::scaling(1, 0.5, 1) * &transformations::rotation_z(std::f32::consts::PI/5.0);
        s.set_transform(&m);
        let y = 2.0_f32.sqrt()/2.0;
        let z = -y;
        let n = s.normal_at(point(0,y,z));
        assert_eq!(n, vector(0, 0.97014, -0.24254));
    }

    #[test]
    fn test_sphere_material() {
        let s = Sphere::new();
        let m = &s.material;
        assert_eq!(*m, Material::default());
    }

    #[test]
    fn test_sphere_material2() {
        let mut s = Sphere::new();
        let mut m = Material::default();
        m.ambient = 1.0;
        s.material = m;
        assert_eq!(s.material,m);
    }
}

// ​Scenario​: A sphere has a default material
// ​ 	  ​Given​ s ← sphere()
// ​ 	  ​When​ m ← s.material
// ​ 	  ​Then​ m = material()
// ​ 	
// ​ 	​Scenario​: A sphere may be assigned a material
// ​ 	  ​Given​ s ← sphere()
// ​ 	    ​And​ m ← material()
// ​ 	    ​And​ m.ambient ← 1
// ​ 	  ​When​ s.material ← m
// ​ 	  ​Then​ s.material = m