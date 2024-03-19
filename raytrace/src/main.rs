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
