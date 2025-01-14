use crate::rays::Ray;
use crate::tuple::{Tuple, origin};
use crate::intersections::{Intersection, intersection, intersections};
use crate::matrix::Matrix;
use crate::shapes::{Base, Shape, shape};
use crate::shape_index::ShapeIndex;
use crate::materials::{Material, material};

#[derive(Debug,PartialEq,Clone)]
pub struct Sphere {
    supe: Base,
}

impl Shape for Sphere {
    fn get_transform(&self) -> &Matrix {
        &self.supe.get_transform()
    }

    fn set_transform(&mut self, t: Matrix){
        self.supe.set_transform( t );
    }

    fn get_material(&self) -> &Material {
        &self.supe.get_material()
    }

    fn set_material(&mut self, m: Material){
        self.supe.set_material( m );
    }

    fn local_normal_at(&self, object_point: Tuple) -> Tuple {
        object_point - origin()
    }

    fn intersect(&self, r: Ray) -> Vec<Intersection> {
        let r2 = self.saved_ray(r);
        let sphere_to_ray = r2.origin - origin();

        let a = r2.direction.dot(&r2.direction);
        let b = 2.0 * ( r2.direction.dot(&sphere_to_ray) );
        let c = (sphere_to_ray.dot(&sphere_to_ray)) - 1.0;

        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec!();
        } else {
            let t1 = (-b - discriminant.sqrt()) / ( 2.0 * a);
            let i1 = intersection(t1, self.get_index());

            let t2 = (-b + discriminant.sqrt()) / ( 2.0 * a);
            let i2 = intersection(t2, self.get_index());

            return intersections(&[i1,i2]);
        }
    }

    fn get_index(&self) -> usize {
        self.supe.get_index()
    }

    fn set_index(&mut self, index: usize){
        self.supe.set_index(index);
    }

    fn get_parent(&self) -> Option<usize> {
        self.supe.get_parent()
    }

    fn set_parent(&mut self, parent_index: usize){
        self.supe.set_parent(parent_index);
    }
    
    fn get_reference(&self) -> ShapeIndex {
        self.supe.get_reference()
    }
    
    fn add_child(&mut self, mut _child: Box<dyn Shape>) -> usize {
        0
    }

    fn get_object(&self, _index: usize) -> Option<&Box<dyn Shape>> {
        None
    }

    fn get_size(&self) -> usize {
        0
    }
}

pub fn sphere() -> Sphere {
    Sphere { 
        supe: shape(),
    }
}

pub fn glass_sphere() -> Sphere {
    let mut s = Sphere {
        supe: shape(),
    };
    let mut mat = material();
    mat.transparency = 1.0;
    mat.refractive_index = 1.5;
    s.set_material(mat);
    s
}

#[cfg(test)]
mod tests {
    use crate::spheres::{sphere, glass_sphere};
    use crate::shapes::Shape;
    use crate::tuple::{point, vector, origin};
    use crate::rays::ray;
    use crate::matrix::identity;
    use crate::transform::{translation, scaling, rotation_z};
    use crate::materials::material;
    use crate::equals::equals;
    use std::f64::consts::{PI, SQRT_2};
    //use std::f64::consts::SQRT_3;  // unfortunately still in experimental branch...
    
    #[test]
    fn new_creates_unique_spheres(){
        let s1 = sphere();
        let s2 = sphere();
        assert_ne!( &s1 as *const _, &s2 as *const _ ); 
    }

    #[test]
    fn ray_intersects_sphere_at_two_points(){
        let s = sphere();
        let r = ray( point( 0.0, 0.0, -5.0 ),
                     vector( 0.0, 0.0, 1.0 ), 
                     0);
        let xs = s.intersect(r);
        assert_eq!( xs.len(), 2 );
        assert_eq!( xs[0].t, 4.0 );
        assert_eq!( xs[1].t, 6.0 );
    }

    #[test]
    fn ray_intersects_sphere_at_tangent(){
        let s = sphere();
        let r = ray( point( 0.0, 1.0, -5.0 ),
                     vector( 0.0, 0.0, 1.0 ), 
                     0);
        let xs = s.intersect(r);
        assert_eq!( xs.len(), 2 );
        assert_eq!( xs[0].t, 5.0 );
        assert_eq!( xs[1].t, 5.0 );
    }

    #[test]
    fn ray_misses_sphere(){
        let s = sphere();
        let r = ray( point( 0.0, 2.0, -5.0 ),
                     vector( 0.0, 0.0, 1.0 ), 
                     0);
        let xs = s.intersect(r);
        assert_eq!( xs.len(), 0 );
    }

    #[test]
    fn ray_originates_inside_sphere(){
        let s = sphere();
        let r = ray( origin(),
                     vector( 0.0, 0.0, 1.0 ), 
                     0);
        let xs = s.intersect(r);
        assert_eq!( xs.len(), 2 );
        assert_eq!( xs[0].t, -1.0 );
        assert_eq!( xs[1].t, 1.0 );
    }

    #[test]
    fn sphere_is_behind_ray(){
        let s = sphere();
        let r = ray( point( 0.0, 0.0, 5.0 ),
                     vector( 0.0, 0.0, 1.0 ), 
                     0);
        let xs = s.intersect(r);
        assert_eq!( xs.len(), 2 );
        assert_eq!( xs[0].t, -6.0 );
        assert_eq!( xs[1].t, -4.0 );
    }

    #[test]
    fn intersect_sets_object_on_intersections(){
        let s = sphere();
        let r = ray( point( 0.0, 0.0, -5.0 ),
                     vector( 0.0, 0.0, 1.0 ), 
                     0);
        let xs = s.intersect(r);
        assert_eq!( xs.len(), 2 );
        assert_eq!( xs[0].object, 0);
        assert_eq!( xs[1].object, 0);
    }

    #[test]
    fn sphere_default_transform(){
        let s = sphere();
        assert!( s.get_transform().equals( identity() ));
    }

    #[test]
    fn changing_sphere_transform(){
        let mut s = sphere();
        let t = translation( 2.0, 3.0, 4.0 );
        s.set_transform( t );
        assert!( s.get_transform().equals( translation( 2.0, 3.0, 4.0 ) ));
    }

    #[test]
    fn intersect_scaled_sphere_with_ray(){
        let mut s = sphere();
        s.set_transform( scaling( 2.0, 2.0, 2.0 ));

        let r = ray( point( 0.0, 0.0, -5.0 ),
                     vector( 0.0, 0.0, 1.0 ), 
                     0);
        let xs = s.intersect(r);

        assert_eq!( xs.len(), 2 );
        assert_eq!( xs[0].t, 3.0 );
        assert_eq!( xs[1].t, 7.0 );
    }

    #[test]
    fn intersect_translated_sphere_with_ray(){
        let mut s = sphere();
        s.set_transform( translation( 5.0, 0.0, 0.0 ));

        let r = ray( point( 0.0, 0.0, -5.0 ),
                     vector( 0.0, 0.0, 1.0 ), 
                     0);
        let xs = s.intersect(r);

        assert_eq!( xs.len(), 0 );
    }

    #[test]
    fn normal_on_sphere_along_x_axis(){
        let s = sphere();
        let n = s.normal_at( point( 1.0, 0.0, 0.0 ));

        assert!( n.equals( vector( 1.0, 0.0, 0.0 )));
    }

    #[test]
    fn normal_on_sphere_along_y_axis(){
        let s = sphere();
        let n = s.normal_at( point( 0.0, 1.0, 0.0 ));

        assert!( n.equals( vector( 0.0, 1.0, 0.0 )));
    }

    #[test]
    fn normal_on_sphere_along_z_axis(){
        let s = sphere();
        let n = s.normal_at( point( 0.0, 0.0, 1.0 ));

        assert!( n.equals( vector( 0.0, 0.0, 1.0 )));
    }

    #[test]
    fn normal_on_sphere_at_nonaxial_point(){
        let sqrt_3: f64 = 3_f64.sqrt();   // see comment above about constant in std::f64
        let s = sphere();
        let n = s.normal_at( point( sqrt_3 / 3.0, sqrt_3 / 3.0, sqrt_3 / 3.0 ));

        assert!( n.equals( vector( sqrt_3 / 3.0, sqrt_3 / 3.0, sqrt_3 / 3.0 )));
    }

    #[test]
    fn normal_is_normalized_vector(){
        let sqrt_3: f64 = 3_f64.sqrt();   // see comment above about constant in std::f64
        let s = sphere();
        let n = s.normal_at( point( sqrt_3 / 3.0, sqrt_3 / 3.0, sqrt_3 / 3.0 ));

        assert!( n.equals( n.normal() ));
    }

    #[test]
    fn normal_on_translated_sphere(){
        let mut s = sphere();
        s.set_transform( translation( 0.0, 1.0, 0.0 ));
        let n = s.normal_at( point( 0.0, 1.70711, -0.70711 ));

        assert!( n.equals( vector( 0.0, 0.70711, -0.70711 )));
    }

    #[test]
    fn normal_on_transformed_sphere(){
        let mut s = sphere();
        let m = scaling( 1.0, 0.5, 1.0 ).mult( &rotation_z( PI / 5.0 ));
        s.set_transform( m );
        let n = s.normal_at( point( 0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0 ));

        assert!( n.equals( vector( 0.0, 0.97014, -0.24254 )));
    }

    #[test]
    fn sphere_has_default_material(){
        let s = sphere();
        assert!( s.get_material().equals( material() ));
    }

    #[test]
    fn sphere_can_be_assigned_material(){
        let mut s = sphere();
        let mut m = material();
        m.ambient = 1.0;
        s.set_material( m.clone() );
        assert!( !s.get_material().equals( material() ));
        assert!( s.get_material().equals( m ));
    }

    #[test]
    fn glassy_sphere_helper_ctor(){
        let s = glass_sphere();
        assert!(s.get_transform().equals(identity()));
        assert!(equals(s.get_material().transparency, 1.0)); 
        assert!(equals(s.get_material().refractive_index, 1.5)); 
    }
}
