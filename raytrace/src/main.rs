use raytrace::tuple::{point, vector};
use std::f64::consts::PI;
use raytrace::camera::camera;
use raytrace::transform::view_transform;
use raytrace::world::default_world;

fn main() {
    let w = default_world();
        
    let mut c = camera(200, 200, PI / 2.0);
    let from = point(0.0, 0.0, -5.0);
    let to = point(0.0, 0.0, 0.0);
    let up = vector(0.0, 1.0, 0.0);
    c.transform = view_transform(from, to, up);

    let image = c.render(w);

    let _ = image.to_ppm("world_render.ppm");
}

