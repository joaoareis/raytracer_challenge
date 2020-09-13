use raytracer::point_vector::{PointVector,vector,point};
use raytracer::canvas::Canvas;
use raytracer::color::Color;
use raytracer::transformations;
use std::fs;
use std::f32::consts::PI;

fn main() {
    let mut cv = Canvas::new(900,550);

    let center = point(450,255,0);
    let mut v = vector(225,0,0); 
    let c = Color::new(255,255,255);
    let mut p = v + center;
    let rotation_matrix = transformations::rotation_z(2.0*PI/12.0);

    for ring in 0..1 {
        for r in 0..12 {
            for i in 0..3 {
                for j in 0..3 {
                    cv.write_pixel_f32(p.x+i as f32, p.y+j as f32, c);
                }
            }
            v = &rotation_matrix * &v;
            p = v + center;

        }
        v = v + vector(10,0,0);
    }

    let ppm = cv.to_ppm();
    fs::write("ch4.ppm",ppm).unwrap();
}