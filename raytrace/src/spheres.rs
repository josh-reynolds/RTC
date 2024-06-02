use crate::rays::Ray;
use crate::tuple::Tuple;

#[derive(Debug)]
pub struct Sphere {
}

impl Sphere {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn intersect(&self, r: Ray) -> Vec<f64> {
        let sphere_to_ray = r.origin.sub( Tuple::origin() );

        let a = r.direction.dot(&r.direction);
        let b = 2.0 * ( r.direction.dot(&sphere_to_ray) );
        let c = (sphere_to_ray.dot(&sphere_to_ray)) - 1.0;

        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec!();
        } else {
            let t1 = (-b - discriminant.sqrt()) / ( 2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / ( 2.0 * a);
            return vec!(t1, t2);
        }

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

    #[test]
    fn ray_intersects_sphere_at_tangent(){
        let s = Sphere::new();
        let r = Ray::new( Tuple::point( Number::from(0),
                                        Number::from(1),
                                        Number::from(-5)),
                          Tuple::vector( Number::from(0),
                                         Number::from(0),
                                         Number::from(1)) );
        let xs = s.intersect(r);
        assert_eq!( xs.len(), 2 );
        assert_eq!( xs[0], 5.0 );
        assert_eq!( xs[1], 5.0 );
    }

    #[test]
    fn ray_misses_sphere(){
        let s = Sphere::new();
        let r = Ray::new( Tuple::point( Number::from(0),
                                        Number::from(2),
                                        Number::from(-5)),
                          Tuple::vector( Number::from(0),
                                         Number::from(0),
                                         Number::from(1)) );
        let xs = s.intersect(r);
        assert_eq!( xs.len(), 0 );
    }

    #[test]
    fn ray_originates_inside_sphere(){
        let s = Sphere::new();
        let r = Ray::new( Tuple::origin(),
                          Tuple::vector( Number::from(0),
                                         Number::from(0),
                                         Number::from(1)) );
        let xs = s.intersect(r);
        assert_eq!( xs.len(), 2 );
        assert_eq!( xs[0], -1.0 );
        assert_eq!( xs[1], 1.0 );
    }

    #[test]
    fn sphere_is_behind_ray(){
        let s = Sphere::new();
        let r = Ray::new( Tuple::point( Number::from(0),
                                        Number::from(0),
                                        Number::from(5)),
                          Tuple::vector( Number::from(0),
                                         Number::from(0),
                                         Number::from(1)) );
        let xs = s.intersect(r);
        assert_eq!( xs.len(), 2 );
        assert_eq!( xs[0], -6.0 );
        assert_eq!( xs[1], -4.0 );
    }
}
