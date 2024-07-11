use raytrace::tuple::{point, vector};
use std::f64::consts::PI;
use raytrace::camera::camera;
use raytrace::transform::{view_transform, translation, rotation_x, rotation_y, scaling};
use raytrace::world::world;
use raytrace::spheres::sphere;
use raytrace::shapes::Shape;
use raytrace::color::color;
use raytrace::materials::material;
use raytrace::lights::point_light;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let mut w = world();
    w.light = Some(point_light( point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0) ));

    let mut floor = sphere();
    floor.set_transform( scaling(10.0, 0.01, 10.0) );
    floor.supe.material = material();
    floor.supe.material.color = color(1.0, 0.9, 0.9);
    floor.supe.material.specular = 0.0;

    let mut left_wall = sphere();
    left_wall.set_transform( translation(0.0, 0.0, 5.0).mult(
                            &rotation_y(-PI / 4.0).mult(
                                &rotation_x(PI / 2.0).mult(
                                    &scaling(10.0, 0.01, 10.0)))));
    left_wall.supe.material = floor.supe.material;

    let mut right_wall = sphere();
    right_wall.set_transform( translation(0.0, 0.0, 5.0).mult(
                            &rotation_y(PI / 4.0).mult(
                                &rotation_x(PI / 2.0).mult(
                                    &scaling(10.0, 0.01, 10.0)))));
    right_wall.supe.material = floor.supe.material;

    let mut middle = sphere();
    middle.set_transform( translation(-0.5, 1.0, 0.5) );
    middle.supe.material = material();
    middle.supe.material.color = color(0.9, 0.1, 0.1);
    middle.supe.material.diffuse = 0.7;
    middle.supe.material.specular = 0.3;

    let mut right = sphere();
    right.set_transform( translation(1.5, 0.5, -0.5).mult(
                          &scaling(0.5, 0.5, 0.5)));
    right.supe.material = material();
    right.supe.material.color = color(0.1, 0.9, 0.1);
    right.supe.material.diffuse = 0.7;
    right.supe.material.specular = 0.3;

    let mut left = sphere();
    left.set_transform( translation(-1.5, 0.33, -0.75).mult(
                          &scaling(0.33, 0.33, 0.33)));
    left.supe.material = material();
    left.supe.material.color = color(0.1, 0.1, 0.9);
    left.supe.material.diffuse = 0.7;
    left.supe.material.specular = 0.3;
    
    w.objects.push(floor);
    w.objects.push(left_wall);
    w.objects.push(right_wall);
    w.objects.push(middle);
    w.objects.push(right);
    w.objects.push(left);

    let mut c = camera(200, 100, PI / 3.0);
    let from = point(0.0, 1.5, -5.0);
    let to = point(0.0, 1.0, 0.0);
    let up = vector(0.0, 1.0, 0.0);
    c.transform = view_transform(from, to, up);

    let image = c.render(w);

    let _ = image.to_ppm("world_shadowed_render.ppm");

    let elapsed = now.elapsed();
    println!("Size: {} x {}", c.hsize, c.vsize);
    println!("Rendering time: {} seconds", elapsed.as_secs());
}
