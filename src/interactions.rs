use crate::shapes::Sphere;
use crate::ray::Ray;
use crate::point_vector::{PointVector,point,vector};


pub fn intersect(s: &Sphere, r: &Ray) -> Vec<f32>{

    let sphere_to_ray = r.origin - s.center;
    let a = r.direction.dot(&r.direction);
    let b = 2.0 * r.direction.dot(&sphere_to_ray);
    let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        vec![]
    }
    else {
        let t1 = (-b - discriminant.sqrt()) / (2.0*a) ;
        let t2 = (-b + discriminant.sqrt()) / (2.0*a) ;
        let mut v = vec![t1,t2];
        v.sort_by(|a,b| a.partial_cmp(b).unwrap());
        v

        
    }

}

#[cfg(test)]
mod tests_shapes {
    use super::*;

    #[test]
    fn test_intersect_1() {
        let r = Ray::new(point(0,0,-5), vector(0, 0, 1));
        let s = Sphere::new();
        let xs = intersect(&s, &r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
        }

    #[test]
    fn test_intersect_2() {
        let r = Ray::new(point(0,1,-5), vector(0, 0, 1));
        let s = Sphere::new();
        let xs = intersect(&s, &r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);
        }

    #[test]
    fn test_intersect_3() {
        let r = Ray::new(point(0,3,-5), vector(0, 0, 1));
        let s = Sphere::new();
        let xs = intersect(&s, &r);

        assert_eq!(xs.len(), 0);
        }

    #[test]
    fn test_intersect_4() {
        let r = Ray::new(point(0,0,0), vector(0, 0, 1));
        let s = Sphere::new();
        let xs = intersect(&s, &r);

        println!("{:?}", xs);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);
        }
    
    #[test]
    fn test_intersect_5() {
        let r = Ray::new(point(0,0,5), vector(0, 0, 1));
        let s = Sphere::new();
        let xs = intersect(&s, &r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
        }

}


