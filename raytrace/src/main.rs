use raytrace::number::Number;
use raytrace::tuple::Tuple;
use raytrace::canvas::Canvas;
use raytrace::color::Color;

fn main() {
    let mut c = Canvas::new(900, 550);

    let start = Tuple::point(Number::from(0),
                             Number::from(1),
                             Number::from(1));
    let velocity = Tuple::vector(Number::from(1),
                                 Number::from(1.8),
                                 Number::from(0)).normal().mult(11.25);
    let mut p = Projectile{ posn: start, vel: velocity };

    let gravity = Tuple::vector(Number::from(0),
                                Number::from(-0.1),
                                Number::from(0));
    let wind = Tuple::vector(Number::from(-0.01),
                             Number::from(0),
                             Number::from(0));
    let e = Environment{ gravity: gravity, wind: wind };
    
    let red = Color{r:1.0,g:0.0,b:0.0};
    while p.posn.y > 0.0 {
       p = tick(&e, &p);
       let x = p.posn.x as i32;
       let y = (c.height as f64 - p.posn.y) as i32;
       println!("{},{}", x, y);
       c.write_pixel(x.try_into().unwrap(),
                     y.try_into().unwrap(),
                     red);
    }
    
    let _ = c.to_ppm("cannon.ppm");
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
