use raytracer::{PointVector,vector,point};

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
    let mut p = Projectile::new(point(0,1,0), vector(1,1,0).normalize());
    let e = Environment::new(vector(0,-0.1,0), vector(-0.01, 0, 0));

    while p.position.y > 0.0 {
        p = tick(&e,p);
        println!("{:?}",p.position);
    }

}



