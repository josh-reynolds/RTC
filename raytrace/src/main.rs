use raytrace::tuple::point;
use raytrace::canvas::Canvas;
use raytrace::color::Color;
//use raytrace::transform::*;
use raytrace::spheres::Sphere;
use raytrace::rays::Ray;

fn main() {
    let ray_origin = point( 0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let half = wall_size / 2.0;

    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f64;

    let mut c = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color{r:1.0, g:0.0, b:0.0};

    let shape = Sphere::new();

    for y in 0..(canvas_pixels-1){
        let world_y = half - pixel_size * y as f64;

        for x in 0..(canvas_pixels-1){
            let world_x = -half + pixel_size * x as f64;
            let position = point(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (position - ray_origin).normal());
            let xs = shape.intersect(ray);

            if xs.len() > 0 {
                c.write_pixel(x, y, color);
            }
        }
    }
    
    let _ = c.to_ppm("flat_render.ppm");
}

