use raytrace::number::Number;
use raytrace::tuple::Tuple;
use raytrace::canvas::Canvas;
use raytrace::color::Color;
use raytrace::transform::*;
use std::f64::consts::PI;

fn main() {
    let mut c = Canvas::new(400, 400);
    let red = Color{r:1.0,g:0.0,b:0.0};

    let p = Tuple::point(Number::from(0),
                         Number::from(0),
                         Number::from(0));
    
    let clock_size = translation(100.0,0.0,0.0);
    let centering = translation(200.0,200.0,0.0);

    for count in 0..12 {
        let hour = rotation_z((PI / 6.0) * count as f64);
        let p2 = centering.mult( &hour.mult( &clock_size )).multup(&p);
        c.write_pixel((p2.x as i32).try_into().unwrap(),
                      (p2.y as i32).try_into().unwrap(),
                      red);
    }
    
    let _ = c.to_ppm("clock.ppm");
}

