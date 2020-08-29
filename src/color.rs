use std::ops::{Add,Sub,Neg,Mul,Div};
use crate::utils::{compare_float,assert_float_eq};

#[derive(PartialOrd, Clone, Copy, Debug)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32
}


impl Color {

    pub fn new(red: impl Into<f64>, green: impl Into<f64>, blue: impl Into<f64>) -> Color{
        Color {
            red: red.into() as f32,
            green: green.into() as f32,
            blue: blue.into() as f32,
        }
    }

    fn add(&self, c : &Color) -> Color {
        Color::new(self.red + c.red, self.green + c.green, self.blue + c.blue)
    }

    fn subtract(&self, c : &Color) -> Color {
        Color::new(self.red - c.red, self.green - c.green, self.blue - c.blue)
    }

    fn negate(&self) -> Color {
        Color::new(-self.red, -self.green, -self.blue)
    }

    fn multiply_scalar(&self, s: impl Into<f64>) -> Color {
        let new_s = s.into() as f32;
        Color::new(self.red*new_s, self.green*new_s, self.blue*new_s)
    }

    fn divide_scalar(&self, s: impl Into<f64>) -> Color {
        let new_s = s.into() as f32;
        Color::new(self.red/new_s, self.green/new_s, self.blue/new_s)
    }

    fn hadamard_product(&self, c : &Color) -> Color {
        Color::new(self.red * c.red, self.green * c.green, self.blue * c.blue)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        compare_float(&self.red, &other.red) && compare_float(&self.green, &other.green) && compare_float(&self.blue, &other.blue)
    }
}
impl Eq for Color {}

impl Add<Color> for Color {

    type Output = Color;
    fn add(self, _rhs: Color) -> Color {
        Color::add(&self, &_rhs)
    
    }
}

impl Sub<Color> for Color {
    type Output = Color;
    fn sub(self, _rhs: Color) -> Color {
        Color::subtract(&self, &_rhs)
    
    }
}

impl Neg for Color {
    type Output = Color;

    fn neg(self) -> Self::Output {
        self.negate()
    }
}

impl Mul<f32> for Color {

    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        self.multiply_scalar(rhs)
    }
}

impl Mul<i32> for Color {

    type Output = Color;

    fn mul(self, rhs: i32) -> Self::Output {
        let frhs = rhs as f32; 
        self.multiply_scalar(frhs)
    }
}

impl Mul<Color> for Color {

    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        self.hadamard_product(&rhs)
    }
}

impl Div<f32> for Color {

    type Output = Color;

    fn div(self, rhs: f32) -> Self::Output {
        self.divide_scalar(rhs)
    }
}

impl Div<i32> for Color {

    type Output = Color;

    fn div(self, rhs: i32) -> Self::Output {
        let frhs = rhs as f32; 
        self.divide_scalar(frhs)
    }
}


#[cfg(test)]
mod tests_color {
    use super::*;

    #[test]
    fn test_compare_float() {
        let c = Color::new(-0.5,0.4,1.7);
        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn test_add() {
        let c1 = Color::new(0.9,0.6, 0.75);
        let c2 = Color::new(0.7,0.1, 0.25);
        assert_eq!(c1+c2, Color::new(1.6,0.7,1.0))

    }
    #[test]
    fn test_sub() {
        let c1 = Color::new(0.9,0.6, 0.75);
        let c2 = Color::new(0.7,0.1, 0.25);
        assert_eq!(c1-c2, Color::new(0.2,0.5,0.5))

    }
    #[test]
    fn test_mulscalar(){
        let c1 = Color::new(0.2,0.3, 0.4);
        assert_eq!(c1*2, Color::new(0.4,0.6,0.8))

    }

    #[test]
    fn test_mul(){
        let c1 = Color::new(1, 0.2, 0.4);
        let c2 = Color::new(0.9, 1, 0.1);
        assert_eq!(c1*c2, Color::new(0.9, 0.2, 0.04))

    }
}