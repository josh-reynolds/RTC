use raytrace::Tuple;
use raytrace::Number;

fn main() {
    let mut p = Projectile{ posn: Tuple::point(Number::from(0), 
                                               Number::from(1), 
                                               Number::from(0)),
                            vel: Tuple::vector(Number::from(1), 
                                               Number::from(1), 
                                               Number::from(0)).normal() };

    let e = Environment{ gravity: Tuple::vector(Number::from(0), 
                                                Number::from(-0.1), 
                                                Number::from(0)),
                         wind: Tuple::vector(Number::from(-0.01),
                                             Number::from(0),
                                             Number::from(0)) };
    
    while p.posn.y > 0.0 {
       p = tick(&e, &p);
       println!("{:?}", p);
    }
}

#[derive(Debug)]
pub struct Projectile {
    pub posn: Tuple,
    pub vel: Tuple,
}

#[derive(Debug)]
pub struct Environment {
    pub gravity: Tuple,
    pub wind: Tuple,
}

fn tick(e: &Environment, p: &Projectile) -> Projectile {
    let posn = p.posn.plus(&p.vel);
    let vel = p.vel.plus(&e.gravity).plus(&e.wind);
    Projectile{ posn: posn, vel: vel }
}
