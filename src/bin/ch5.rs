use raytracer::point_vector::{vector,point};
use raytracer::canvas::Canvas;
use raytracer::ray::Ray;
use raytracer::shapes::Sphere;
use raytracer::transformations;
use raytracer::interactions::{intersect,Intersections};
use raytracer::color::Color;
use std::fs;

fn main() {
    let pixels = 300;
    let wall_size = 7.0;
    let half = wall_size/2.0;
    let pixel_size = wall_size as f32/ pixels as f32;
    let mut cv = Canvas::new(pixels,pixels);
    let s = Sphere::new();
    let c = Color::new(1,0,0);

    for x in 0..pixels {
        for y in 0..pixels {
            let w_x = -half + x as f32 * pixel_size;
            let w_y = half - y as f32 * pixel_size;
            let v = vector(w_x,w_y,15);
            let r = Ray::new(point(0,0,-5), v);
            let h = Intersections::new_from_intersect(&s,&r);
            if h.hit() != None {
                cv.write_pixel(x ,y,c)

            }
        }
    }

    let ppm = cv.to_ppm();
    fs::write("ch5.ppm",ppm).unwrap();


}