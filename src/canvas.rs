use crate::color::Color;

struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>
}

impl Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        let pixels = vec![vec![Color::new(0,0,0); width]; height];
        Canvas {
            width,
            height,
            pixels
        }
    }

    fn write_pixel(&mut self, i: usize, j: usize, pixel: Color) {
        self.pixels[i][j] = pixel
    }

    fn get_pixel(&self, i: usize, j: usize) -> Color {
        self.pixels[i][j]
    }

    fn to_ppm(&self) -> String {
        let s = format!("PPM\n{} {}\n255", self.width, self.height);
        
    }
}

#[cfg(test)]
mod tests_color {
    use super::*;

    #[test]
    fn test_createcanvas() {
        let c = Color::new(0,0,0);
        let cv = Canvas::new(10,20);
        for row in cv.pixels.iter() {
            for pixel in row {
                assert_eq!(*pixel, c);
            }
        }
    }

    #[test]
    fn test_write() {
        let mut cv = Canvas::new(10,20);
        let red = Color::new(1,0,0);
        let green = Color::new(0,1,0);
        cv.write_pixel(2,3, red);
        assert_eq!(cv.get_pixel(2,3), red);
        assert_ne!(cv.get_pixel(2,3), green);
    }

    #[test]
    fn test_toppm() {
        let mut cv = Canvas::new(10,20);
        let s = cv.to_ppm();
        assert_eq!(s, "PPM\n10 20\n255")
    }
}