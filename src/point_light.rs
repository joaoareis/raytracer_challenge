use crate::point_vector::PointVector;
use crate::color::Color;


pub struct PointLight {
    intensity: Color,
    position: PointVector
}

impl PointLight {
    pub fn new(position: PointVector, intensity: Color) -> PointLight {
        PointLight {
            intensity,
            position
        }
    }
}



#[cfg(test)]
mod tests_point_light {
    use super::*;
    use crate::point_vector::{point};

    #[test]
    fn test_new_pl() {
        let intensity = Color::new(1,1,1);
        let position = point(0, 0, 0);
        let light = PointLight::new(position, intensity);
        assert_eq!(position, light.position);
        assert_eq!(intensity, light.intensity);
    }
}