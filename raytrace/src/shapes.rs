use crate::matrix::{Matrix, identity};
use crate::materials::{Material, material};

// Current concrete 'class' is Sphere:
//   Sphere.transform         OK
//   Sphere.material          OK
//   Sphere.set_transform()   OK
//   Sphere.intersect()
//   Sphere.normal_at()
//   sphere()

#[derive(Debug,PartialEq)]
pub struct Base {
    pub transform: Matrix,
    pub material: Material,
}

impl Shape for Base {
    fn set_transform(&mut self, t: Matrix){
        self.transform = t
    }

    fn get_transform(&self) -> &Matrix {
        &self.transform
    }
}

pub trait Shape {
    fn set_transform(&mut self, t: Matrix);
    fn get_transform(&self) -> &Matrix;
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
        assert!( s.material.equals( material() ));
    }

    #[test]
    fn assigning_a_material(){
        let mut s = shape();
        let mut m = material();
        m.ambient = 1.0;
        s.material = m;
        assert!( !s.material.equals( material() ));
        assert!( s.material.equals( m ));
    }
}
