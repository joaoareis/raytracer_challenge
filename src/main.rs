use std::ops::{Add,Sub,Neg,Mul,Div};

fn main() {
    println!("Hello, world!");
}

fn compare_float(a: f32, b: f32) -> bool {
    if (a - b).abs() < 0.0001 {
        true
    } else {
        false
    }
}

fn assert_float_eq(a: f32, b: f32){
    assert_eq!(compare_float(a,b), true)
}

fn vectori(x: i32, y: i32, z: i32) -> PointVector{
    PointVector::new_vector_fromint(x,y,z)
}

fn vectorf(x: f32, y: f32, z: f32) -> PointVector{
    PointVector::new_vector(x,y,z)
}
fn pointi(x: i32, y: i32, z: i32) -> PointVector{
    PointVector::new_point_fromint(x,y,z)
}

fn pointf(x: f32, y: f32, z: f32) -> PointVector{
    PointVector::new_point(x,y,z)
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
struct PointVector {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

impl PointVector {

    fn new(x: f32, y: f32, z: f32, w: f32) -> PointVector{
        PointVector {
            x,
            y,
            z,
            w
        }
    }

    fn new_fromint(x: i32, y: i32, z: i32, w: i32) -> PointVector{

        PointVector::new(x as f32,y as f32,z as f32,w as f32)
    }

    fn new_vector(x: f32, y: f32, z: f32) -> PointVector{
        PointVector {
            x,
            y,
            z,
            w: 0.0
        }
    }

    fn new_vector_fromint(x: i32, y: i32, z: i32) -> PointVector{
        PointVector::new_vector(x as f32,y as f32,z as f32)
    }

    fn new_point_fromint(x: i32, y: i32, z: i32) -> PointVector{
        PointVector::new_point(x as f32,y as f32,z as f32)
    }

    fn new_point(x: f32, y: f32, z: f32) -> PointVector{
        PointVector {
            x,
            y,
            z,
            w: 1.0
        }
    }
    fn is_point(&self) -> bool {
        if self.w == 1.0 {
            true
        }
        else  {
            false
        }
    }

    fn is_vector(&self) -> bool {
        if self.w == 0.0 {
            true
        }
        else  {
            false
        }
    }

    fn add(t1: &PointVector, t2: &PointVector) -> PointVector {
        PointVector {
            x: t1.x + t2.x,
            y: t1.y + t2.y,
            z: t1.z + t2.z,
            w: t1.w + t2.w,

        }
    }

    fn negate(&self) -> PointVector {
        PointVector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,

        }
    }

    fn subtract(t1: &PointVector, t2: &PointVector) -> PointVector {
        PointVector {
            x: t1.x - t2.x,
            y: t1.y - t2.y,
            z: t1.z - t2.z,
            w: t1.w - t2.w,

        }
    }

    fn multiply_scalar(&self, s: f32) -> PointVector {
       PointVector {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
            w: self.w * s,

        }
    }

    fn divide_scalar(&self, s: f32) -> PointVector {
        PointVector {
             x: self.x / s,
             y: self.y / s,
             z: self.z / s,
             w: self.w / s,
 
         }
     }

     fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
     }

     fn normalize(&self) -> PointVector {
        let m = self.magnitude();
        Self {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
            w: self.w

        }
     }
}

impl Add<PointVector> for PointVector {

    type Output = PointVector;
    fn add(self, _rhs: PointVector) -> PointVector {
        PointVector::add(&self, &_rhs)
    
    }
}

impl Sub<PointVector> for PointVector {
    type Output = PointVector;
    fn sub(self, _rhs: PointVector) -> PointVector {
        PointVector::subtract(&self, &_rhs)
    
    }
}

impl Neg for PointVector {
    type Output = PointVector;

    fn neg(self) -> Self::Output {
        self.negate()
    }
}

impl Mul<f32> for PointVector {

    type Output = PointVector;

    fn mul(self, rhs: f32) -> Self::Output {
        self.multiply_scalar(rhs)
    }
}

impl Mul<i32> for PointVector {

    type Output = PointVector;

    fn mul(self, rhs: i32) -> Self::Output {
        let frhs = rhs as f32; 
        self.multiply_scalar(frhs)
    }
}

impl Div<f32> for PointVector {

    type Output = PointVector;

    fn div(self, rhs: f32) -> Self::Output {
        self.divide_scalar(rhs)
    }
}

impl Div<i32> for PointVector {

    type Output = PointVector;

    fn div(self, rhs: i32) -> Self::Output {
        let frhs = rhs as f32; 
        self.divide_scalar(frhs)
    }
}

#[cfg(test)]
mod tests_pointvector {
    use super::*;

    #[test]
    fn test_compare_float() {
        assert_eq!(compare_float(6.0,7.0), false);
        assert_eq!(compare_float(6.0,6.0000000001), true);
    }

    #[test]
    fn test1() {
        let pv = PointVector {x: 4.3,y: -4.2,z: 3.1,w: 1.0};
        assert_eq!(pv.x, 4.3);
        assert_eq!(pv.y, -4.2);
        assert_eq!(pv.z, 3.1);
        assert_eq!(pv.w, 1.0);

        assert_eq!(pv.is_point(), true);
        assert_eq!(!pv.is_vector(), true);
    }

    #[test]
    fn test2() {
        let pv = PointVector {x: 4.3,y: -4.2,z: 3.1,w: 0.0};
        assert_eq!(pv.x, 4.3);
        assert_eq!(pv.y, -4.2);
        assert_eq!(pv.z, 3.1);
        assert_eq!(pv.w, 0.0);

        assert_eq!(!pv.is_point(), true);
        assert_eq!(pv.is_vector(), true);
    }

    #[test]
    fn test_new_vector(){
        let v = PointVector::new_vector(4.0,-4.0,3.0);
        assert_eq!(v.x, 4.0);
        assert_eq!(v.y, -4.0);
        assert_eq!(v.z, 3.0);
        assert_eq!(v.w, 0.0);

    }

    #[test]
    fn test_new_point(){
        let v = PointVector::new_point(4.0,-4.0, 3.0);
        assert_eq!(v.x, 4.0);
        assert_eq!(v.y, -4.0);
        assert_eq!(v.z, 3.0);
        assert_eq!(v.w, 1.0);
        assert_eq!(v.is_point(), true);

    }
    
    #[test]
    fn test_eq(){
        let v1 = PointVector::new_point(4.0,-4.0, 3.0);
        let v2 = PointVector::new_point(4.0,-4.0, 3.0);
        assert_eq!(v1==v2, true);

    }

    #[test]
    fn test_eq_false(){
        let v1 = PointVector::new_point(4.0,-4.0, 3.0);
        let v2 = PointVector::new_point(4.0,-3.0, 3.0);
        assert_eq!(v1==v2, false);

    }

    #[test]
    fn test_add_pointvector() {
        let v1 = PointVector::new(3.0,-2.0,5.0,1.0);
        let v2 = PointVector {x: -2.0,y: 3.0,z: 1.0,w: 0.0};
        let sum_v = PointVector::add(&v1, &v2);
        let expected_sum_v = PointVector::new_fromint(1,1,6,1);
        assert_eq!(expected_sum_v==sum_v, true);
    }

    #[test]
    fn test_subtract_pointvector() {
        let p1 = PointVector::new_point(3.0,2.0,1.0);
        let p2 = PointVector::new_point(5.0, 6.0,7.0);
        let v = PointVector::subtract(&p1,&p2);
        let expected_vector =  PointVector::new_vector(-2.0, -4.0, -6.0);
        let non_expected_vector =  PointVector::new_vector(-2.0, -3.0, -6.0);
        assert_eq!(v==expected_vector, true);
        assert_eq!(v==non_expected_vector, false);
    }

    #[test]
    fn test_subtract_pv_2() {
        let p = PointVector::new_point_fromint(3,2,1);
        let v = PointVector::new_vector_fromint(5,6,7);
        let p2 = p - v;
        let expected_p2 = PointVector::new_point_fromint(-2,-4,-6);
        let non_expected_p2 = PointVector::new_point_fromint(-2,-5,-6);
        assert_eq!(p2==expected_p2, true);
        assert_eq!(p2==non_expected_p2, false);
    }

    #[test]
    fn test_subtract_pv_3() {
        let p = PointVector::new_vector_fromint(3,2,1);
        let v = PointVector::new_vector_fromint(5,6,7);
        let p2 = p - v;
        let expected_p2 = PointVector::new_vector_fromint(-2,-4,-6);
        let non_expected_p2 = PointVector::new_vector_fromint(-2,-5,-6);
        assert_eq!(p2==expected_p2, true);
        assert_eq!(p2==non_expected_p2, false);
    }

    #[test]
    fn test_negate() {
        let zero = PointVector::new_vector_fromint(0,0,0);
        let v = PointVector::new_vector_fromint(1,-2,3);
        let p = zero - v;
        let expected_p = PointVector::new_vector_fromint(-1,2,-3);
        let non_expected_p = PointVector::new_vector_fromint(-2,-5,-6);
        assert_eq!(p==expected_p, true);
        assert_eq!(p==non_expected_p, false);
    }

    #[test]
    fn test_negate2() {
        let v = PointVector::new_fromint(1,-2,3, -4);
        let v2 = v.negate();
        let expected_v2 = PointVector::new_fromint(-1,2,-3, 4);
        assert_eq!(v2==expected_v2, true);
    }

    #[test]
    fn test_negate3() {
        let v = PointVector::new_fromint(1,-2,3, -4);
        let v2 = -v;
        let expected_v2 = PointVector::new_fromint(-1,2,-3, 4);
        assert_eq!(v2==expected_v2, true);
    }

    #[test]
    fn test_mul() {
        let v = PointVector::new_fromint(1,-2,3, -4);
        assert_eq!((v*3.5)==PointVector::new(3.5,-7.0,10.5, -14.0), true);
        assert_eq!((v*3.5)==PointVector::new(3.5,-7.0,10.5, -14.1), false);
    }

    #[test]
    fn test_mul_2() {
        let v = PointVector::new_fromint(1,-2,3, -4);
        assert_eq!((v*0.5)==PointVector::new(0.5,-1.0,1.5,-2.0), true);

    }

    #[test]
    fn test_div() {
        let v = PointVector::new_fromint(1,-2,3, -4);
        assert_eq!((v/2)==PointVector::new(0.5,-1.0,1.5,-2.0), true);

    }

    #[test]
    fn test_magnitude() {
        assert_eq!(PointVector::new_vector_fromint(1,0,0).magnitude(), 1.0);
        assert_eq!(PointVector::new_vector_fromint(0,1,0).magnitude(), 1.0);
        assert_eq!(PointVector::new_vector_fromint(0,0,1).magnitude(), 1.0);
        assert_eq!(PointVector::new_vector_fromint(1,2,3).magnitude(), (14.0_f32).sqrt());
        assert_eq!(PointVector::new_vector_fromint(-1,-2,-3).magnitude(), (14.0_f32).sqrt());
    }

    #[test]
    fn test_normalize() {
        assert_eq!(vectori(4,0,0).normalize(), vectori(1,0,0));

        let sqrt_14 = 14.0_f32.sqrt();
        assert_eq!(vectori(1,2,3).normalize(), vectorf(1.0/sqrt_14, 2.0/sqrt_14, 3.0/sqrt_14));
        assert_float_eq(vectori(1,2,3).normalize().magnitude(), 1.0);

    }

}

