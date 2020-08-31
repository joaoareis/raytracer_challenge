use raytracer::point_vector::{PointVector,vector,point};
use raytracer::canvas::Canvas;
use raytracer::color::Color;
use std::fs;

struct Projectile {
    position: PointVector,
    velocity: PointVector
}

impl Projectile {
    fn new(position: PointVector, velocity: PointVector) -> Projectile {
        Projectile {
            position,
            velocity
        }
    }

    fn x(&self) -> f32 {
        self.position.x
    }

    fn y(&self) -> f32 {
        self.position.y
    }
}

struct Environment {
    gravity: PointVector,
    wind: PointVector
}

impl Environment {
    fn new(gravity: PointVector, wind: PointVector) -> Environment {
        Environment {
            gravity,
            wind
        }
    }
}

fn tick(e: &Environment, p: Projectile) -> Projectile{
    let new_pos = p.position + p.velocity;
    let new_velocity = p.velocity + e.gravity + e.wind;
    Projectile::new(new_pos, new_velocity)
}

fn main(){
    let mut cv = Canvas::new(900,550);
    let mut p = Projectile::new(point(0,1,0), vector(1,3,0).normalize()*11.25);
    let e = Environment::new(vector(0,-0.1,0), vector(-0.01, 0, 0));

    loop {
        let c = Color::new(1,0,0);
        p = tick(&e,p);
        if p.position.y < 0.0 {
            break;
        }
        let projectile_height = p.y().round() as usize;
        let real_height = match projectile_height {
            p if p <= cv.height => cv.height - projectile_height,
            _ => cv.height
        }; 
        let real_width = p.x().round() as usize; 
        cv.write_pixel(real_width, real_height, c);
    }
    let ppm = cv.to_ppm();
    fs::write("test.ppm",ppm);
    

}





