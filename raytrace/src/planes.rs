use crate::shapes::{Base, Shape, shape};
use crate::shape_index::ShapeIndex;
use crate::intersections::{Intersection, intersection, intersections};
use crate::tuple::{Tuple, vector};
use crate::rays::Ray;
use crate::materials::Material;
use crate::matrix::Matrix;
use crate::equals::EPSILON;

#[derive(Debug,PartialEq)]
pub struct Plane {
    supe: Base,
}

impl Shape for Plane {
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

    fn local_normal_at(&self, _object_point: Tuple) -> Tuple {
        vector(0.0, 1.0, 0.0)
    }

    fn intersect(&self, r: Ray) -> Vec<Intersection> {
        let r2 = self.saved_ray(r);
        if r2.direction.y.abs() < EPSILON {
            return vec!();
        } 

        let t = -r2.origin.y / r2.direction.y;
        let i = intersection(t, self.get_index());

        return intersections(&[i]);
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

pub fn plane() -> Plane {
    Plane { 
        supe: shape(),
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::{point, vector};
    use crate::planes::plane;
    use crate::shapes::Shape;
    use crate::rays::ray;

    #[test]
    fn normal_of_plane_is_constant_everywhere(){
        let p = plane();
        let n1 = p.local_normal_at( point( 0.0, 0.0,   0.0) );
        let n2 = p.local_normal_at( point(10.0, 0.0, -10.0) );
        let n3 = p.local_normal_at( point(-5.0, 0.0, 150.0) );

        assert!( n1 == vector(0.0, 1.0, 0.0) );
        assert!( n2 == vector(0.0, 1.0, 0.0) );
        assert!( n3 == vector(0.0, 1.0, 0.0) );
    }

    #[test]
    fn intersect_with_ray_parallel_to_plane(){
        let p = plane();
        let r = ray( point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0), 0 );

        let xs = p.intersect( r );

        assert!( xs.len() == 0 );
    }

    #[test]
    fn intersect_with_coplanar_ray(){
        let p = plane();
        let r = ray( point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0), 0 );

        let xs = p.intersect( r );

        assert!( xs.len() == 0 );
    }

    #[test]
    fn ray_intersecting_plane_from_above(){
        let p = plane();
        let r = ray( point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0), 0 );

        let xs = p.intersect( r );

        assert!( xs.len() == 1 );
        assert!( xs[0].t == 1.0 );
        assert!( xs[0].object == 0 );
    }

    #[test]
    fn ray_intersecting_plane_from_below(){
        let p = plane();
        let r = ray( point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0), 0 );

        let xs = p.intersect( r );

        assert!( xs.len() == 1 );
        assert!( xs[0].t == 1.0 );
        assert!( xs[0].object == 0 );
    }
}
