use crate::rays::Ray;
use crate::tuple::Tuple;
use crate::intersections::Intersection;
use crate::matrix::Matrix;

#[derive(Debug)]
pub struct Sphere {
    pub transform: Matrix,
}

impl<'a> Sphere {
    pub fn new() -> Self {
        Self { transform: Matrix::identity() }
    }
    
    pub fn intersect(&'a self, r: Ray) -> Vec<Intersection<'a>> {
        let r2 = r.transform( self.transform.inverse() );
        let sphere_to_ray = r2.origin - Tuple::origin();

        let a = r2.direction.dot(&r2.direction);
        let b = 2.0 * ( r2.direction.dot(&sphere_to_ray) );
        let c = (sphere_to_ray.dot(&sphere_to_ray)) - 1.0;

        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec!();
        } else {
            let t1 = (-b - discriminant.sqrt()) / ( 2.0 * a);
            let i1 = Intersection::new(t1, &self);

            let t2 = (-b + discriminant.sqrt()) / ( 2.0 * a);
            let i2 = Intersection::new(t2, &self);

            return Intersection::intersections(&[i1,i2]);
        }
    }

    pub fn set_transform(&mut self, t: Matrix){
        self.transform = t
    }
}

#[cfg(test)]
mod tests {
    use crate::spheres::Sphere;
    use crate::number::Number;
    use crate::tuple::Tuple;
    use crate::rays::Ray;
    use crate::matrix::Matrix;
    use crate::transform::translation;
    use crate::transform::scaling;

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
        assert_eq!( xs[0].t, 4.0 );
        assert_eq!( xs[1].t, 6.0 );
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
        assert_eq!( xs[0].t, 5.0 );
        assert_eq!( xs[1].t, 5.0 );
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
        assert_eq!( xs[0].t, -1.0 );
        assert_eq!( xs[1].t, 1.0 );
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
        assert_eq!( xs[0].t, -6.0 );
        assert_eq!( xs[1].t, -4.0 );
    }

    #[test]
    fn intersect_sets_object_on_intersections(){
        let s = Sphere::new();
        let r = Ray::new( Tuple::point( Number::from(0),
                                        Number::from(0),
                                        Number::from(-5)),
                          Tuple::vector( Number::from(0),
                                         Number::from(0),
                                         Number::from(1)) );
        let xs = s.intersect(r);
        assert_eq!( xs.len(), 2 );
        assert_eq!( xs[0].object as *const _, &s as *const _);
        assert_eq!( xs[1].object as *const _, &s as *const _);
    }

    #[test]
    fn sphere_default_transform(){
        let s = Sphere::new();
        assert!( s.transform.equals( Matrix::identity() ));
    }

    #[test]
    fn changing_sphere_transform(){
        let mut s = Sphere::new();
        let t = translation(2.0, 3.0, 4.0);
        s.set_transform( t );
        assert!( s.transform.equals( translation(2.0, 3.0, 4.0) ));
    }

    #[test]
    fn intersect_scaled_sphere_with_ray(){
        let mut s = Sphere::new();
        s.set_transform( scaling( 2.0, 2.0, 2.0 ));

        let r = Ray::new( Tuple::point( Number::from(0),
                                        Number::from(0),
                                        Number::from(-5)),
                          Tuple::vector( Number::from(0),
                                         Number::from(0),
                                         Number::from(1)) );
        let xs = s.intersect(r);

        assert_eq!( xs.len(), 2 );
        assert_eq!( xs[0].t, 3.0 );
        assert_eq!( xs[1].t, 7.0 );
    }

    #[test]
    fn intersect_translated_sphere_with_ray(){
        let mut s = Sphere::new();
        s.set_transform( translation( 5.0, 0.0, 0.0 ));

        let r = Ray::new( Tuple::point( Number::from(0),
                                        Number::from(0),
                                        Number::from(-5)),
                          Tuple::vector( Number::from(0),
                                         Number::from(0),
                                         Number::from(1)) );
        let xs = s.intersect(r);

        assert_eq!( xs.len(), 0 );
    }
}
