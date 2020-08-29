use crate::color::Color;

fn clamp(ipt: i32, min: i32, max: i32) -> i32 {
    if ipt > max {
        max
    } else if ipt < min {
        min
    } else {
        ipt
    }
}
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Vec<Color>>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let pixels = vec![vec![Color::new(0,0,0); width]; height];
        Canvas {
            width,
            height,
            pixels
        }
    }

    fn set_all(&mut self, c: Color) {
        for w in 0..self.width {
            for h in 0..self.height {
                self.write_pixel(w, h, c.clone())
            }
        }
    }

    pub fn write_pixel(&mut self, w: usize, h: usize, pixel: Color) {
        if (h <= self.height) && (w <= self.width) {
        self.pixels[h][w] = pixel
        }
    }

    fn get_pixel(&self, w: usize, h: usize) -> Color {
        self.pixels[h][w]
    }

    pub fn to_ppm(&self) -> String {
        let mut s = String::new();
        s.push_str("P3");
        s.push('\n');
        s.push_str(&format!("{} {}", self.width, self.height));
        s.push('\n');
        s.push_str("255");
        s.push('\n');
        let mut row_size = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                let c = self.pixels[i][j];
                let red = clamp((c.red * 255.0).round() as i32, 0, 255).to_string();
                if (red.len() + row_size) > 70 {
                    s.push_str("\n");
                    s.push_str(&red);
                    s.push_str(" ");
                    row_size = red.len();
                } else if (red.len() + row_size) == 70 {
                    s.push_str(&red);
                    s.push_str("\n");
                    row_size = 0;
                } else  {
                    s.push_str(&red);
                    s.push_str(" ");
                    row_size = row_size + red.len();
                }
                let green = clamp((c.green * 255.0).round() as i32, 0, 255).to_string();
                if (green.len() + row_size) > 70 {
                    s.push_str("\n");
                    s.push_str(&green);
                    s.push_str(" ");
                    row_size = green.len();
                } else if (green.len() + row_size) == 70 {
                    s.push_str(&green);
                    s.push_str("\n");
                    row_size = 0;
                } else  {
                    s.push_str(&green);
                    s.push_str(" ");
                    row_size = row_size + green.len();
                }
                let blue = clamp((c.blue * 255.0).round() as i32, 0, 255).to_string();
                if (blue.len() + row_size) > 70 {
                    s.push_str("\n");
                    s.push_str(&blue);
                    s.push_str(" ");
                    row_size = blue.len();
                } else if (blue.len() + row_size) == 70 {
                    s.push_str(&blue);
                    s.push_str("\n");
                    row_size = 0;
                } else  {
                    s.push_str(&blue);
                    s.push_str(" ");
                    row_size = row_size + blue.len();
                }
            }
            // Remove the last space
            s.pop();
            s.push('\n');
        }
        s
        
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
        let cv = Canvas::new(10,20);
        let s = cv.to_ppm();
        let vec_s : Vec<&str> = s.split('\n').collect();
        let slice_s = vec_s[0..3].join("\n");
        assert_eq!(slice_s, "P3\n10 20\n255")
    }

    #[test]
    fn test_toppm_2() {
        let mut cv = Canvas::new(5,3);
        let c1 = Color::new(1.5, 0, 0);
        let c2 = Color::new(0, 0.5, 0);
        let c3 = Color::new(-0.5, 0, 1);
        cv.write_pixel(0, 0, c1);
        cv.write_pixel(2, 1, c2);
        cv.write_pixel(4, 2, c3);
        let s = cv.to_ppm();
        let vec_s : Vec<&str> = s.split('\n').collect();
        let slice_s = vec_s[3..6].join("\n");
        assert_eq!(slice_s, "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255")
    }

    #[test]
    fn test_toppm3() {
        let mut cv = Canvas::new(10,2);
        let c = Color::new(1,0.8,0.6);
        cv.set_all(c);
        let s = cv.to_ppm();
        let vec_s : Vec<&str> = s.split('\n').collect();
        let slice_s = vec_s[3..8].join("\n");
        assert_eq!(slice_s,"255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 \n153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 \n204 153 255 204 153 255 204 153 255 204 153 255 204 153\n")
    }

    #[test]
    fn test_toppm4() {
        let cv = Canvas::new(5,3);
        let mut s = cv.to_ppm();
        assert_eq!(s.pop().unwrap(),'\n')
    }
}



