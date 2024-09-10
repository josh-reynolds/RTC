use raytrace::tuple::{point, vector};
use std::f64::consts::PI;
use raytrace::camera::camera;
use raytrace::transform::{view_transform, translation, scaling, 
                          rotation_x, rotation_y};
use raytrace::world::world;
use raytrace::spheres::sphere;
use raytrace::planes::plane;
use raytrace::shapes::Shape;
use raytrace::color::color;
use raytrace::materials::material;
use raytrace::lights::point_light;
use raytrace::patterns::Pattern;
use raytrace::stripes::stripe_pattern;
use raytrace::cubes::cube;
use raytrace::cylinders::cylinder;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let mut w = world();
    w.light = Some(point_light( point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0) ));

    let mut floor = plane();
    let mut mat = material();
    mat.color = color(0.5, 0.5, 0.5);
    mat.reflective = 0.5;
    floor.set_material( mat );
    w.add_object(Box::new(floor));

    let mut wall = plane();
    wall.set_transform(rotation_x(PI/2.0).mult(
                        &translation(0.0, 15.0, 0.0)));
    let mut mat = material();
    mat.color = color(0.9, 0.8, 0.7);
    wall.set_material(mat);
    w.add_object(Box::new(wall));

    let mut middle = sphere();
    middle.set_transform( translation(-0.5, 1.0, 0.5) );
    let mut mat = material();
    mat.reflective = 0.8;
    mat.color = color(0.1, 0.1, 0.1);
    mat.diffuse = 0.7;
    mat.specular = 0.3;
    middle.set_material( mat );
    w.add_object(Box::new(middle));

    let mut right = sphere();
    right.set_transform( translation(1.5, 0.5, -0.5).mult(
                          &scaling(0.5, 0.5, 0.5)));
    let mut mat = material();
    mat.color = color(0.0, 0.0, 0.4);
    mat.diffuse = 0.1;
    mat.specular = 1.0;
    mat.shininess = 300.0;
    mat.transparency = 0.8;
    mat.refractive_index = 1.5;
    mat.reflective = 0.9;
    right.set_material( mat );
    w.add_object(Box::new(right));

    let mut left = sphere();
    left.set_transform( translation(-1.5, 0.33, -0.75).mult(
                          &scaling(0.33, 0.33, 0.33)));
    let mut mat = material();
    let mut p4 = stripe_pattern(color(1.0, 0.0, 0.0), color(1.0, 1.0, 0.0));
    p4.set_pattern_transform(scaling(0.5, 0.5, 0.5));
    let current = w.add_pattern( Box::new(p4) );
    mat.pattern = Some(current);
    mat.color = color(0.1, 0.1, 0.9);
    mat.diffuse = 0.7;
    mat.specular = 0.3;
    left.set_material( mat );
    w.add_object(Box::new(left));
    
    let mut cu = cube();
    cu.set_transform(translation(2.0, 2.0, 2.0).mult(
                     &scaling(0.5, 0.5, 0.5).mult(
                     &rotation_x(PI/2.0).mult(
                     &rotation_y(PI/3.0)))));
    let mut mat = material();
    mat.color = color(0.5, 0.0, 0.5);
    mat.reflective = 0.5;
    cu.set_material(mat);
    w.add_object(Box::new(cu));

    let mut cyl = cylinder();
    cyl.minimum = 1.0;
    cyl.maximum = 2.0;
    cyl.closed = true;
    cyl.set_transform(translation(-2.0, 0.0, 0.0).mult(
                      &scaling(0.5, 0.5, 0.5)));
    let mut mat = material();
    mat.color = color(0.8, 0.8, 0.0);
    mat.shininess = 200.0;
    mat.reflective = 0.2;
    cyl.set_material(mat);
    w.add_object(Box::new(cyl));

    let mut c = camera(600, 300, PI / 3.0);
    let from = point(0.0, 1.5, -5.0);
    let to = point(0.0, 1.0, 0.0);
    let up = vector(0.0, 1.0, 0.0);
    c.transform = view_transform(from, to, up);

    let image = c.render(w);

    let _ = image.to_ppm("cylinder_closed.ppm");

    let elapsed = now.elapsed();
    println!("Size: {} x {}", c.hsize, c.vsize);
    println!("Rendering time: {} seconds", elapsed.as_secs());
}
