use crate::tuple::Tuple;
use crate::matrix::Matrix;

#[derive(Debug,Clone,Copy)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn position(&self, distance: f64) -> Tuple {
        self.origin + (*&self.direction * distance)
    }

    pub fn transform(&self, t: Matrix) -> Self {
        ray( t.multup( &self.origin ),
             t.multup( &self.direction ))
    }
}

pub fn ray(origin: Tuple, direction: Tuple) -> Ray {
    Ray { origin, direction }
}

#[cfg(test)]
mod tests {
    use crate::rays::ray;
    use crate::tuple::{point, vector};
    use crate::transform::{translation, scaling};

    #[test]
    fn new_creates_rays(){
        let o = point( 1.0, 2.0, 3.0);
        let d = vector( 4.0, 5.0, 6.0);

        let r = ray(o, d);
        assert!( r.origin.equals(point( 1.0, 2.0, 3.0))) ;
        assert!( r.direction.equals(vector( 4.0, 5.0, 6.0))) ;
    }

    #[test]
    fn computing_point_from_distance(){
        let o = point( 2.0, 3.0, 4.0);
        let d = vector( 1.0, 0.0, 0.0);

        let r = ray(o, d);
        assert!( r.position(0.0).equals(point( 2.0, 3.0, 4.0))) ;
        assert!( r.position(1.0).equals(point( 3.0, 3.0, 4.0))) ;
        assert!( r.position(-1.0).equals(point( 1.0, 3.0, 4.0))) ;
        assert!( r.position(2.5).equals(point( 4.5, 3.0, 4.0))) ;

    }

    #[test]
    fn translating_a_ray(){
        let o = point( 1.0, 2.0, 3.0);
        let d = vector( 0.0, 1.0, 0.0);
        let r = ray(o, d);

        let m = translation(3.0, 4.0, 5.0);
        let r2 = r.transform(m);

        assert!( r2.origin.equals(point( 4.0, 6.0, 8.0))) ;
        assert!( r2.direction.equals(vector( 0.0, 1.0, 0.0))) ;
    }

    #[test]
    fn scaling_a_ray(){
        let o = point( 1.0, 2.0, 3.0);
        let d = vector( 0.0, 1.0, 0.0);
        let r = ray(o, d);

        let m = scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(m);

        assert!( r2.origin.equals(point( 2.0, 6.0, 12.0))) ;
        assert!( r2.direction.equals(vector( 0.0, 3.0, 0.0))) ;
    }
}
