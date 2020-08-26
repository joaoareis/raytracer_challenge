fn main() {
    println!("Hello, world!");
}

fn compare_float(a: &f32, b: &f32) -> bool {
    if (*a - *b).abs() < 0.0001 {
        true
    } else {
        false
    }
}

#[derive(PartialEq, PartialOrd)]
struct PointVector {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

impl PointVector {
    fn new_vector(x: f32, y: f32, z: f32) -> PointVector{
        PointVector {
            x,
            y,
            z,
            w: 0.0
        }
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
}

#[cfg(test)]
mod tests_pointvector {
    use super::*;

    #[test]
    fn test_compare_float() {
        assert_eq!(compare_float(&6.0,&7.0), false);
        assert_eq!(compare_float(&6.0,&6.0000000001), true);
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
}