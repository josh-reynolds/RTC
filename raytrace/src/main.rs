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
    let mut mat = material();
    mat.color = color(1.0, 0.9, 0.9);
    mat.specular = 0.0;
    floor.set_material( mat );

    let mut left_wall = sphere();
    left_wall.set_transform( translation(0.0, 0.0, 5.0).mult(
                            &rotation_y(-PI / 4.0).mult(
                                &rotation_x(PI / 2.0).mult(
                                    &scaling(10.0, 0.01, 10.0)))));
    left_wall.set_material( *floor.get_material() );

    let mut right_wall = sphere();
    right_wall.set_transform( translation(0.0, 0.0, 5.0).mult(
                            &rotation_y(PI / 4.0).mult(
                                &rotation_x(PI / 2.0).mult(
                                    &scaling(10.0, 0.01, 10.0)))));
    right_wall.set_material( *floor.get_material() );

    let mut middle = sphere();
    middle.set_transform( translation(-0.5, 1.0, 0.5) );
    let mut mat = material();
    mat.color = color(0.9, 0.1, 0.1);
    mat.diffuse = 0.7;
    mat.specular = 0.3;
    middle.set_material( mat );

    let mut right = sphere();
    right.set_transform( translation(1.5, 0.5, -0.5).mult(
                          &scaling(0.5, 0.5, 0.5)));
    let mut mat = material();
    mat.color = color(0.1, 0.9, 0.1);
    mat.diffuse = 0.7;
    mat.specular = 0.3;
    right.set_material( mat );

    let mut left = sphere();
    left.set_transform( translation(-1.5, 0.33, -0.75).mult(
                          &scaling(0.33, 0.33, 0.33)));
    let mut mat = material();
    mat.color = color(0.1, 0.1, 0.9);
    mat.diffuse = 0.7;
    mat.specular = 0.3;
    left.set_material( mat );
    
    w.add(Box::new(floor));
    w.add(Box::new(left_wall));
    w.add(Box::new(right_wall));
    w.add(Box::new(middle));
    w.add(Box::new(right));
    w.add(Box::new(left));
    
    let mut c = camera(200, 100, PI / 3.0);
    let from = point(0.0, 1.5, -5.0);
    let to = point(0.0, 1.0, 0.0);
    let up = vector(0.0, 1.0, 0.0);
    c.transform = view_transform(from, to, up);

    let image = c.render(w);

    let _ = image.to_ppm("index_refactored.ppm");

    let elapsed = now.elapsed();
    println!("Size: {} x {}", c.hsize, c.vsize);
    println!("Rendering time: {} seconds", elapsed.as_secs());
}
