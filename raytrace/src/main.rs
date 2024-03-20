use raytrace::Tuple;

fn main() {
    println!("Hello, world!");
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

fn tick(e: Environment, p: Projectile) -> Projectile {
    let posn = p.posn.plus(&p.vel);
    let vel = p.vel.plus(&e.gravity).plus(&e.wind);
    Projectile{ posn: posn, vel: vel }
}
