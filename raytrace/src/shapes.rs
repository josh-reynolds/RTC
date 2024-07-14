use crate::matrix::{Matrix, identity};
use crate::materials::{Material, material};
use crate::tuple::{Tuple, vector};
use crate::rays::Ray;
use crate::intersections::Intersection;

// Current concrete 'class' is Sphere:
//   Sphere.transform         OK
//   Sphere.material          OK
//   Sphere.set_transform()   OK
//   Sphere.normal_at()       OK
//   Sphere.intersect()       OK
//   sphere()

#[derive(Debug,PartialEq)]
pub struct Base {
    transform: Matrix,
    material: Material,
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


    fn normal_at(&self, _world_point: Tuple) -> Tuple {
        vector(0.0, 0.0, 0.0)
    }

    fn intersect<'a>(&'a self, _r: Ray) -> Vec<Intersection<'a>> {
        vec!()
    }
}

pub trait Shape {
    fn get_transform(&self) -> &Matrix;
    fn set_transform(&mut self, t: Matrix);
    fn get_material(&self) -> &Material;
    fn set_material(&mut self, m: Material);
    fn normal_at(&self, world_point: Tuple) -> Tuple;
    fn intersect<'a>(&'a self, r: Ray) -> Vec<Intersection<'a>>; 
}

pub fn shape() -> Base {
    Base {
        transform: identity(),
        material: material(),
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::identity;
    use crate::shapes::{Shape, shape};
    use crate::transform::translation;
    use crate::materials::material;

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
        s.material = m;
        assert!( !s.get_material().equals( material() ));
        assert!( s.get_material().equals( m ));
    }
}
