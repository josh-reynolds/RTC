use raytrace::tuple::{point, vector};
use std::f64::consts::PI;
use raytrace::camera::camera;
use raytrace::transform::{view_transform, translation, scaling, 
                          rotation_x, rotation_z};
use raytrace::world::world;
use raytrace::spheres::sphere;
use raytrace::planes::plane;
use raytrace::shapes::Shape;
use raytrace::color::color;
use raytrace::materials::material;
use raytrace::lights::point_light;
use raytrace::cylinders::cylinder;
use raytrace::cones::cone;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let mut w = world();
    w.light = Some(point_light( point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0) ));

    let mut floor = plane();
    let mut mat = material();
    mat.color = color(0.7, 0.4, 0.7);
    floor.set_material( mat );
    w.add_object(Box::new(floor));

    let mut wall = plane();
    wall.set_transform(rotation_x(PI/2.0).mult(
                       &translation(0.0, 15.0, 0.0)));
    let mut mat = material();
    mat.color = color(0.9, 0.8, 0.7);
    wall.set_material(mat);
    w.add_object(Box::new(wall));

    let mut eraser = sphere();
    eraser.set_transform(translation(1.0, 1.0, 0.0).mult(
                         &scaling(0.5, 0.5, 0.5)));
    let mut mat = material();
    mat.color = color(0.68, 0.24, 0.51);
    mat.diffuse = 0.7;
    mat.specular = 0.3;
    eraser.set_material( mat );
    w.add_object(Box::new(eraser));
    
    let mut cyl = cylinder();
    cyl.minimum = -2.0;
    cyl.maximum = 2.0;
    cyl.closed = true;
    cyl.set_transform(translation(0.0, 1.0, 0.0).mult(
                      &rotation_z(PI/2.0).mult(
                      &scaling(0.5, 0.5, 0.5))));
    let mut mat = material();
    mat.color = color(0.73, 0.64, 0.08);
    mat.diffuse = 0.7;
    mat.specular = 0.3;
    cyl.set_material(mat);
    w.add_object(Box::new(cyl));

    let mut con = cone();
    con.minimum = -1.0;
    con.maximum =  0.0;
    con.set_transform(translation(-2.0, 1.0, 0.0).mult(
                      &rotation_z(PI/2.0).mult(
                      &scaling(0.5, 1.0, 0.5))));
    let mut mat = material();
    mat.color = color(0.35, 0.35, 0.35);
    mat.diffuse = 0.7;
    mat.specular = 0.3;
    con.set_material(mat);
    w.add_object(Box::new(con));

    let mut c = camera(600, 300, PI / 3.0);
    let from = point(0.0, 1.5, -5.0);
    let to = point(0.0, 1.0, 0.0);
    let up = vector(0.0, 1.0, 0.0);
    c.transform = view_transform(from, to, up);

    let image = c.render(w);

    let _ = image.to_ppm("pencil.ppm");

    let elapsed = now.elapsed();
    println!("Size: {} x {}", c.hsize, c.vsize);
    println!("Rendering time: {} seconds", elapsed.as_secs());
}
