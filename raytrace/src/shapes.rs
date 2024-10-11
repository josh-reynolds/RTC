use crate::matrix::{Matrix, identity};
use crate::materials::{Material, material};
use crate::tuple::{Tuple, vector, point};
use crate::rays::Ray;
use crate::intersections::Intersection;
use crate::shape_index::{ShapeIndex, shape_index};
use core::fmt::Debug;

#[derive(Debug,PartialEq,Clone)]
pub struct Base {
    transform: Matrix,
    material: Material,
    i: ShapeIndex,
}

impl Shape for Base {
    fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    fn set_transform(&mut self, t: Matrix){
        self.transform = t
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn set_material(&mut self, m: Material){
        self.material = m
    }

    fn local_normal_at(&self, object_point: Tuple) -> Tuple {
        vector(object_point.x, object_point.y, object_point.z)
    }

    fn intersect(&self, _r: Ray) -> Vec<Intersection> {
        vec!()
    }

    fn get_index(&self) -> usize {
        self.i.get_index()
    }

    fn set_index(&mut self, index: usize){
        self.i.set_index(index);
    }

    fn get_parent(&self) -> Option<usize> {
        self.i.parent
    }

    fn set_parent(&mut self, parent_index: usize){
        self.i.parent = Some(parent_index);
    }
}

pub trait Shape {
    fn get_transform(&self) -> &Matrix;
    fn set_transform(&mut self, t: Matrix);
    fn get_material(&self) -> &Material;
    fn set_material(&mut self, m: Material);
    
    fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = self.get_transform().inverse().multup( &world_point );
        let local_normal = self.local_normal_at( object_point );
        let mut world_normal = self.get_transform()
                                   .inverse()
                                   .transpose()
                                   .multup( &local_normal );
        world_normal.w = 0.0;
        world_normal.normal()
    }
    fn local_normal_at(&self, object_point: Tuple) -> Tuple;
    fn intersect(&self, r: Ray) -> Vec<Intersection>; 
    fn get_index(&self) -> usize;
    fn set_index(&mut self, index: usize);
    fn get_parent(&self) -> Option<usize>;
    fn set_parent(&mut self, parent_index: usize);

    // text implements this as a mutable field on Shape,
    // but this causes mutability contagion across the entire
    // project - trying this out as a query method instead
    // will probably want to figure out a better name here
    fn saved_ray(&self, r: Ray) -> Ray {
        r.transform( self.get_transform().inverse() )
    }

    fn world_to_object(&self, _p: Tuple) -> Tuple {
        //match self.get_parent() {
            //Some(parent) => parent.world_to_object(p),
            //None         => self.get_transform().inverse().multup(&p)
        //}
        point(0.0, 0.0, -1.0)
    }
}

impl Debug for dyn Shape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Shape {}", self.get_index())
    }
}

impl PartialEq for dyn Shape {
    fn eq(&self, _: &(dyn Shape + 'static)) -> bool {
        true   // need to research right way to compare trait objects
    }
}

pub fn shape() -> Base {
    Base {
        transform: identity(),
        material: material(),
        i: shape_index(),
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::identity;
    use crate::shapes::{Shape, shape};
    use crate::transform::{scaling, translation, rotation_y, rotation_z};
    use crate::materials::material;
    use crate::rays::ray;
    use crate::tuple::{point, vector};
    use crate::groups::group;
    use crate::spheres::sphere;
    use std::f64::consts::{PI, SQRT_2};

    #[test]
    fn shape_default_transformation(){
        let s = shape();
        assert!( s.get_transform().equals( identity() ));
    }

    #[test]
    fn assigning_a_transformation(){
        let mut s = shape();
        let t = translation( 2.0, 3.0, 4.0 );
        s.set_transform( t );
        assert!( s.get_transform().equals( translation( 2.0, 3.0, 4.0 ) ));
    }

    #[test]
    fn shape_default_material(){
        let s = shape();
        assert!( s.get_material().equals( material() ));
    }

    #[test]
    fn assigning_a_material(){
        let mut s = shape();
        let mut m = material();
        m.ambient = 1.0;
        s.set_material( m.clone() );
        assert!( !s.get_material().equals( material() ));
        assert!( s.get_material().equals( m ));
    }

    #[test]
    fn intersecting_scaled_shape_with_ray(){
        let r = ray( point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 0 );
        let mut s = shape();
        s.set_transform( scaling(2.0, 2.0, 2.0) );

        let local_ray = s.saved_ray(r);
        
        assert!( local_ray.origin.equals( point(0.0, 0.0, -2.5) ));
        assert!( local_ray.direction.equals( vector(0.0, 0.0, 0.5) ));
    }

    #[test]
    fn intersecting_translated_shape_with_ray(){
        let r = ray( point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 0 );
        let mut s = shape();
        s.set_transform( translation(5.0, 0.0, 0.0) );

        let local_ray = s.saved_ray(r);

        assert!( local_ray.origin.equals( point(-5.0, 0.0, -5.0) ));
        assert!( local_ray.direction.equals( vector(0.0, 0.0, 1.0) ));
    }

    #[test]
    fn computing_normal_on_translated_shape(){
        let mut s = shape();
        s.set_transform( translation(0.0, 1.0, 0.0) );

        let n = s.normal_at( point(0.0, 1.70711, -0.70711) );
        assert!( n.equals( vector(0.0, 0.70711, -0.70711) ));
    }

    #[test]
    fn computing_normal_on_transformed_shape(){
        let mut s = shape();
        s.set_transform( scaling(1.0, 0.5, 1.0).mult( &rotation_z( PI / 5.0 ) ));

        let n = s.normal_at( point(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0) );
        assert!( n.equals( vector(0.0, 0.97014, -0.24254) ));
    }

    #[test]
    fn a_shape_has_a_parent_attribute(){
        let s = shape();
        assert!(s.i.parent == None);
    }

    #[test]
    fn converting_point_from_world_to_object_space(){
        let mut g1 = group();
        g1.set_transform(rotation_y(PI/2.0));

        let mut g2 = group();
        g2.set_transform(scaling(2.0, 2.0, 2.0));

        let mut s = sphere();
        s.set_transform(translation(5.0, 0.0, 0.0));

        g2.add_child(Box::new(s.clone()));
        g1.add_child(Box::new(g2));

        let p = s.world_to_object(point(-2.0, 0.0, -10.0));
        assert!(p.equals(point(0.0, 0.0, -1.0)));
    }
}
