use crate::matrix::{Matrix, identity};
use crate::materials::{Material, material};

// Current concrete 'class' is Sphere:
//   Sphere.transform         OK
//   Sphere.material          OK
//   Sphere.intersect()
//   Sphere.set_transform()   OK
//   Sphere.normal_at()
//   sphere()

#[derive(Debug)]
pub struct Base {
    pub transform: Matrix,
    pub material: Material,
}

impl Shape for Base {
    fn set_transform(&mut self, t: Matrix){
        self.transform = t
    }
}

pub trait Shape {
    fn set_transform(&mut self, t: Matrix);
}

pub fn test_shape() -> Base {
    Base {
        transform: identity(),
        material: material(),
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::identity;
    use crate::shapes::{Shape, test_shape};
    use crate::transform::translation;
    use crate::materials::material;

    #[test]
    fn shape_default_transformation(){
        let s = test_shape();
        assert!( s.transform.equals( identity() ));
    }

    #[test]
    fn assigning_a_transformation(){
        let mut s = test_shape();
        let t = translation( 2.0, 3.0, 4.0 );
        s.set_transform( t );
        assert!( s.transform.equals( translation( 2.0, 3.0, 4.0 ) ));
    }

    #[test]
    fn shape_default_material(){
        let s = test_shape();
        assert!( s.material.equals( material() ));
    }

    #[test]
    fn assigning_a_material(){
        let mut s = test_shape();
        let mut m = material();
        m.ambient = 1.0;
        s.material = m;
        assert!( !s.material.equals( material() ));
        assert!( s.material.equals( m ));
    }
}
