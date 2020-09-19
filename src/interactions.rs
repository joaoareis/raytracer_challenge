use crate::shapes::{Sphere,Shape};
use crate::ray::Ray;
use crate::utils::compare_float;
use crate::point_vector::{PointVector,point,vector,reflect};
use crate::point_light::PointLight;
use crate::color::Color;
use crate::material::Material;


pub struct Intersections<'a> {
    pub v: Vec<Intersection<'a>>
}

impl<'a> Intersections<'a> {
    pub fn new(v: Vec<Intersection>) -> Intersections {
        Intersections{v}
    }

    pub fn sort(&mut self) {
        self.v.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
    }

    pub fn get_intersections_vector(&self) -> & Vec<Intersection>{
        &self.v
    }

    pub fn new_from_intersect(s: &'a Sphere, r: &Ray) -> Intersections<'a> {
        let i = intersect(s, r);
        Self::new(i)
    }

    pub fn hit(self) -> Option<Intersection<'a>> {
        if self.v.len() == 0 {
            return None
        }
        let mut curr_hit = self.v[0];
        for i in self.v.iter() {
            if curr_hit.t > i.t || (curr_hit.t <0.0 && i.t > 0.0){
                curr_hit = *i;
            }
        }
        if curr_hit.t < 0.0 {
            None
        }
        else {
            Some(curr_hit)
        }
    }
}

#[derive(Debug,Clone,Copy)]
pub struct Intersection<'a> {
    pub t: f32,
    pub obj: &'a Sphere
}

impl<'a> Intersection<'a> {
    pub fn new(t: impl Into<f64>, obj: &Sphere) -> Intersection {
        Intersection {t: t.into() as f32, obj}
    }
}

impl PartialEq for Intersection<'_> {
    fn eq(&self, other: &Self) -> bool {
        compare_float(&self.t, &other.t) && (self.obj == other.obj)
        
    }
}
impl Eq for Intersection<'_> {}

pub fn intersect<'a>(s: &'a Sphere, r: &Ray) -> Vec<Intersection<'a>>{
    let r = r.transform(&s.transform.inverse());
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
        vec![Intersection::new(v[0],&s), Intersection::new(v[1],&s)]

        
    }

}


pub fn lighting(m: Material, light: PointLight, position: PointVector, eyev: PointVector, normalv: PointVector) -> Color {
    let effective_color = m.color * light.intensity;
    let mut specular : Color;
    let diffuse : Color;
    let ambient = effective_color * m.ambient;
    
    let light_vector = (light.position - position).normalize();
    let cos_light_normal = light_vector.dot(&normalv);//(light_vector.magnitude()*normalv.magnitude());
    if cos_light_normal < 0.0 {
        specular = Color::new(0, 0, 0);
        diffuse = Color::new(0, 0, 0);
    }
    else {
        diffuse = effective_color * m.diffuse * cos_light_normal;
    

        let reflect_vector = reflect(light_vector.negate(), normalv);
        let cos_reflect_eye = reflect_vector.dot(&eyev);//(reflect_vector.magnitude() * eyev.magnitude());
    
        if cos_reflect_eye <= 0.0 {
            specular = Color::new(0,0,0);

        }

        else {
            specular = light.intensity * m.specular * cos_reflect_eye.powf(m.shiness);
        }
    }
    
    //println!("{:?} {:?} {:?} {:?}", ambient, diffuse, specular, eyev.magnitude());
    ambient + diffuse + specular
}


#[cfg(test)]
mod tests_shapes {
    use super::*;
    use crate::transformations;

    #[test]
    fn test_intersect_1() {
        let r = Ray::new(point(0,0,-5), vector(0, 0, 1));
        let s = Sphere::new();
        let xs = intersect(&s, &r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
        }

    #[test]
    fn test_intersect_2() {
        let r = Ray::new(point(0,1,-5), vector(0, 0, 1));
        let s = Sphere::new();
        let xs = intersect(&s, &r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
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
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
        }
    
    #[test]
    fn test_intersect_5() {
        let r = Ray::new(point(0,0,5), vector(0, 0, 1));
        let s = Sphere::new();
        let xs = intersect(&s, &r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
        }

    #[test]
    fn test_intersect_6() {
        let r = Ray::new(point(0,0,5), vector(0, 0, 1));
        let s = Sphere::new();
        let xs = intersect(&s, &r);

        assert_eq!(xs.len(), 2);
        assert_eq!(*xs[0].obj, s);
        assert_eq!(*xs[1].obj, s);
        assert_eq!(std::ptr::eq(xs[0].obj,&s),true);
        }
    
    #[test]
    fn test_intersection_new(){
        let t = 3.5;
        let s = Sphere::new();

        let p = point(1, 1, 1);
        let mut s2 = Sphere::new();
        s2.center = p;
        let i = Intersection::new(t, &s);
        assert_eq!(i.t,t);
        assert_eq!(*i.obj, s);
        assert_ne!(*i.obj, s2);
    }

    #[test]
    fn test_intersections(){
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i1,i2];
        assert_eq!(xs.len(),2);
        assert_eq!(xs[0].t,1.0);
        assert_eq!(xs[1].t,2.0);
    }

    #[test]
    fn test_intersections2(){
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i1,i2];
        assert_eq!(xs.len(),2);
        assert_eq!(xs[0].t,1.0);
        assert_eq!(xs[1].t,2.0);
    }

    #[test]
    fn test_hit1() {
        let s = Sphere::new();
        let i1 = Intersection::new(1, &s);
        let i2 = Intersection::new(2, &s);
        let mut xs = Intersections::new(vec![i1,i2]);
        let i = xs.hit().unwrap();
        assert_eq!(i,i1)
    }

    #[test]
    fn test_hit2() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1, &s);
        let i2 = Intersection::new(1, &s);
        let mut xs = Intersections::new(vec![i1,i2]);
        let i = xs.hit().unwrap();
        assert_eq!(i,i2)
    }

    #[test]
    fn test_hit3() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2, &s);
        let i2 = Intersection::new(-1, &s);
        let mut xs = Intersections::new(vec![i1,i2]);
        let i = xs.hit();
        assert_eq!(i,None)
    }

    #[test]
    fn test_hit4() {
        let s = Sphere::new();
        let i1 = Intersection::new(5, &s);
        let i2 = Intersection::new(7, &s);
        let i3 = Intersection::new(-3, &s);
        let i4 = Intersection::new(3, &s);
        let mut xs = Intersections::new(vec![i1,i2,i3,i4]);
        let i = xs.hit().unwrap();
        assert_eq!(i,i4)
    }

    #[test]
    fn test_hit5() {
        let mut xs = Intersections::new(vec![]);
        let i = xs.hit();
        assert_eq!(i, None)

    }

    #[test]
    fn test_intersection_scaling() {
        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));
        let mut s = Sphere::new();
        s.set_transform(&transformations::scaling(2, 2, 2));
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn test_intersection_translation() {
        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));
        let mut s = Sphere::new();
        s.set_transform(&transformations::translate(5, 0, 0));
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_lighting_1() {
        let m = Material::default();
        let position = point(0, 0, 0);
        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = PointLight::new(point(0, 0, -10),Color::new(1,1,1));
        let result = lighting(m, light, position, eyev, normalv);
        assert_eq!(result, Color::new(1.9,1.9,1.9))
    }

    #[test]
    fn test_lighting_2() {
        let m = Material::default();
        let position = point(0, 0, 0);
        let eyev = vector(0, 2_f32.sqrt()/2_f32, -2_f32.sqrt()/2_f32);
        let normalv = vector(0, 0, -1);
        let light = PointLight::new(point(0, 0, -10),Color::new(1,1,1));
        let result = lighting(m, light, position, eyev, normalv);
        assert_eq!(result, Color::new(1.0,1.0,1.0))
    }

    #[test]
    fn test_lighting_3() {
        let m = Material::default();
        let position = point(0, 0, 0);
        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = PointLight::new(point(0, 10, -10),Color::new(1,1,1));
        let result = lighting(m, light, position, eyev, normalv);
        assert_eq!(result, Color::new(0.7364,0.7364,0.7364))
    }

    #[test]
    fn test_lighting_4() {
        let m = Material::default();
        let position = point(0, 0, 0);
        let eyev = vector(0, -2_f32.sqrt()/2_f32, -2_f32.sqrt()/2_f32);
        let normalv = vector(0, 0, -1);
        let light = PointLight::new(point(0, 10, -10),Color::new(1,1,1));
        let result = lighting(m, light, position, eyev, normalv);
        assert_eq!(result, Color::new(1.6364,1.6364,1.6364))
    }

    #[test]
    fn test_lighting_5() {
        let m = Material::default();
        let position = point(0, 0, 0);
        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = PointLight::new(point(0, 0, 10),Color::new(1,1,1));
        let result = lighting(m, light, position, eyev, normalv);
        assert_eq!(result, Color::new(0.1,0.1,0.1))
    }

}
