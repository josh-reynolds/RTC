use raytrace::tuple::point;
use raytrace::canvas::canvas;
use raytrace::color::color;
use raytrace::spheres::sphere;
use raytrace::rays::Ray;
use raytrace::materials::{material, lighting};
use raytrace::lights::point_light;

fn main() {
    let ray_origin = point( 0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let half = wall_size / 2.0;

    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f64;
    let mut c = canvas(canvas_pixels, canvas_pixels);

    let mut shape = sphere();
    shape.material = material();
    shape.material.color = color(1.0, 0.2, 1.0);

    let light_pos = point(-10.0, 10.0, -10.0);
    let light_color = color(1.0, 1.0, 1.0);
    let light = point_light(light_pos, light_color);

    for y in 0..(canvas_pixels-1){
        let world_y = half - pixel_size * y as f64;

        for x in 0..(canvas_pixels-1){
            let world_x = -half + pixel_size * x as f64;
            let position = point(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (position - ray_origin).normal());
            let xs = shape.intersect(ray);

            if xs.len() > 0 {
                let hit = xs[0];
                let p = ray.position( hit.t );
                let normal = hit.object.normal_at( p );
                let eye = -ray.direction;
                let col = lighting(hit.object.material, &light, p, eye, normal);
                c.write_pixel(x, y, col);
            }
        }
    }
    
    let _ = c.to_ppm("phong_render.ppm");
}

