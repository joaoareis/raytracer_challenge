use raytracer::point_vector::{vector,point};
use raytracer::canvas::Canvas;
use raytracer::ray::Ray;
use raytracer::shapes::Sphere;
use raytracer::transformations;
use raytracer::interactions::{intersect,Intersections,Intersection,lighting};
use raytracer::color::Color;
use raytracer::material::Material;
use raytracer::point_light::PointLight;
use std::fs;

fn main() {
    let pixels = 200;
    let wall_size = 7.0;
    let half = wall_size/2.0;
    let pixel_size = wall_size as f32/ pixels as f32;
    let mut cv = Canvas::new(pixels,pixels);
    let mut s = Sphere::new();
    s.material = Material::default();
    s.material.color = Color::new(1,0.2,1);
    let t = transformations::scaling(1, 0.1, 0.1);
    //s.set_transform(&t);
    let c = Color::new(1,0,0);
    let light = PointLight::new(point(-10,10,-10), Color::new(1,1,1));

    for x in 0..pixels {
        for y in 0..pixels {
            let w_x = -half + x as f32 * pixel_size;
            let w_y = half - y as f32 * pixel_size;
            let v = vector(w_x,w_y,5).normalize();
            let r = Ray::new(point(0,0,-15), v);
            let h = Intersections::new_from_intersect(&s,&r);
            let hit = h.hit();

            if hit != None {
                let p = r.position(hit.unwrap().t);
                let normal = hit.unwrap().obj.normal_at(p);
                let eye = r.direction;
                let c = lighting(hit.unwrap().obj.material, light,p,eye,normal);
                cv.write_pixel(x ,y,c)

            }
        }
    }

    let ppm = cv.to_ppm();
    fs::write("ch5.ppm",ppm).unwrap();


}