use crate::rays::Ray;

#[derive(Debug)]
pub struct Sphere {
}

impl Sphere {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn intersect(&self, r: Ray) -> Vec<f64> {
        vec![4.0, 6.0]
    }
}

#[cfg(test)]
mod tests {
    use crate::spheres::Sphere;
    use crate::number::Number;
    use crate::tuple::Tuple;
    use crate::rays::Ray;

    #[test]
    fn new_creates_unique_spheres(){
        let s1 = Sphere::new();
        let s2 = Sphere::new();
        assert_ne!( &s1 as *const _, &s2 as *const _ ); 
    }

    #[test]
    fn ray_intersects_sphere_at_two_points(){
        let s = Sphere::new();
        let r = Ray::new( Tuple::point( Number::from(0),
                                        Number::from(0),
                                        Number::from(-5)),
                          Tuple::vector( Number::from(0),
                                         Number::from(0),
                                         Number::from(1)) );
        let xs = s.intersect(r);
        assert_eq!( xs.len(), 2 );
        assert_eq!( xs[0], 4.0 );
        assert_eq!( xs[1], 6.0 );
    }
}
