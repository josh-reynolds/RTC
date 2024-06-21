use crate::rays::Ray;
use crate::tuple::Tuple;
use crate::tuple::origin;
use crate::intersections::Intersection;
use crate::matrix::Matrix;
use crate::materials::{Material, material};

#[derive(Debug)]
pub struct Sphere {
    pub transform: Matrix,
    pub material: Material,
}

impl<'a> Sphere {
    pub fn new() -> Self {
        Self { 
            transform: Matrix::identity(),
            material: material(),
        }
    }
    
    pub fn intersect(&'a self, r: Ray) -> Vec<Intersection<'a>> {
        let r2 = r.transform( self.transform.inverse() );
        let sphere_to_ray = r2.origin - origin();

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

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = self.transform.inverse().multup( &world_point );
        let object_normal = object_point - origin();
        let mut world_normal = self.transform.inverse().transpose().multup( &object_normal );
        world_normal.w = 0.0;
        world_normal.normal()
    }
}

#[cfg(test)]
mod tests {
    use crate::spheres::Sphere;
    use crate::tuple::{point, vector, origin};
    use crate::rays::Ray;
    use crate::matrix::Matrix;
    use crate::transform::{translation, scaling, rotation_z};
    use crate::materials::material;
    //use std::f64::consts::SQRT_3;  // unfortunately still in experimental branch...
    use std::f64::consts::PI;
    use std::f64::consts::SQRT_2;
    
    #[test]
    fn new_creates_unique_spheres(){
        let s1 = Sphere::new();
        let s2 = Sphere::new();
        assert_ne!( &s1 as *const _, &s2 as *const _ ); 
    }

    #[test]
    fn ray_intersects_sphere_at_two_points(){
        let s = Sphere::new();
        let r = Ray::new( point( 0.0, 0.0, -5.0 ),
                          vector( 0.0, 0.0, 1.0 ));
        let xs = s.intersect(r);
        assert_eq!( xs.len(), 2 );
        assert_eq!( xs[0].t, 4.0 );
        assert_eq!( xs[1].t, 6.0 );
    }

    #[test]
    fn ray_intersects_sphere_at_tangent(){
        let s = Sphere::new();
        let r = Ray::new( point( 0.0, 1.0, -5.0 ),
                          vector( 0.0, 0.0, 1.0 ));
        let xs = s.intersect(r);
        assert_eq!( xs.len(), 2 );
        assert_eq!( xs[0].t, 5.0 );
        assert_eq!( xs[1].t, 5.0 );
    }

    #[test]
    fn ray_misses_sphere(){
        let s = Sphere::new();
        let r = Ray::new( point( 0.0, 2.0, -5.0 ),
                          vector( 0.0, 0.0, 1.0 ));
        let xs = s.intersect(r);
        assert_eq!( xs.len(), 0 );
    }

    #[test]
    fn ray_originates_inside_sphere(){
        let s = Sphere::new();
        let r = Ray::new( origin(),
                          vector( 0.0, 0.0, 1.0 ));
        let xs = s.intersect(r);
        assert_eq!( xs.len(), 2 );
        assert_eq!( xs[0].t, -1.0 );
        assert_eq!( xs[1].t, 1.0 );
    }

    #[test]
    fn sphere_is_behind_ray(){
        let s = Sphere::new();
        let r = Ray::new( point( 0.0, 0.0, 5.0 ),
                          vector( 0.0, 0.0, 1.0 ));
        let xs = s.intersect(r);
        assert_eq!( xs.len(), 2 );
        assert_eq!( xs[0].t, -6.0 );
        assert_eq!( xs[1].t, -4.0 );
    }

    #[test]
    fn intersect_sets_object_on_intersections(){
        let s = Sphere::new();
        let r = Ray::new( point( 0.0, 0.0, -5.0 ),
                          vector( 0.0, 0.0, 1.0 ));
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
        let t = translation( 2.0, 3.0, 4.0 );
        s.set_transform( t );
        assert!( s.transform.equals( translation( 2.0, 3.0, 4.0 ) ));
    }

    #[test]
    fn intersect_scaled_sphere_with_ray(){
        let mut s = Sphere::new();
        s.set_transform( scaling( 2.0, 2.0, 2.0 ));

        let r = Ray::new( point( 0.0, 0.0, -5.0 ),
                          vector( 0.0, 0.0, 1.0 ));
        let xs = s.intersect(r);

        assert_eq!( xs.len(), 2 );
        assert_eq!( xs[0].t, 3.0 );
        assert_eq!( xs[1].t, 7.0 );
    }

    #[test]
    fn intersect_translated_sphere_with_ray(){
        let mut s = Sphere::new();
        s.set_transform( translation( 5.0, 0.0, 0.0 ));

        let r = Ray::new( point( 0.0, 0.0, -5.0 ),
                          vector( 0.0, 0.0, 1.0 ));
        let xs = s.intersect(r);

        assert_eq!( xs.len(), 0 );
    }

    #[test]
    fn normal_on_sphere_along_x_axis(){
        let s = Sphere::new();
        let n = s.normal_at( point( 1.0, 0.0, 0.0 ));

        assert!( n.equals( vector( 1.0, 0.0, 0.0 )));
    }

    #[test]
    fn normal_on_sphere_along_y_axis(){
        let s = Sphere::new();
        let n = s.normal_at( point( 0.0, 1.0, 0.0 ));

        assert!( n.equals( vector( 0.0, 1.0, 0.0 )));
    }

    #[test]
    fn normal_on_sphere_along_z_axis(){
        let s = Sphere::new();
        let n = s.normal_at( point( 0.0, 0.0, 1.0 ));

        assert!( n.equals( vector( 0.0, 0.0, 1.0 )));
    }

    #[test]
    fn normal_on_sphere_at_nonaxial_point(){
        let sqrt_3: f64 = 3_f64.sqrt();   // see comment above about constant in std::f64
        let s = Sphere::new();
        let n = s.normal_at( point( sqrt_3 / 3.0, sqrt_3 / 3.0, sqrt_3 / 3.0 ));

        assert!( n.equals( vector( sqrt_3 / 3.0, sqrt_3 / 3.0, sqrt_3 / 3.0 )));
    }

    #[test]
    fn normal_is_normalized_vector(){
        let sqrt_3: f64 = 3_f64.sqrt();   // see comment above about constant in std::f64
        let s = Sphere::new();
        let n = s.normal_at( point( sqrt_3 / 3.0, sqrt_3 / 3.0, sqrt_3 / 3.0 ));

        assert!( n.equals( n.normal() ));
    }

    #[test]
    fn normal_on_translated_sphere(){
        let mut s = Sphere::new();
        s.set_transform( translation( 0.0, 1.0, 0.0 ));
        let n = s.normal_at( point( 0.0, 1.70711, -0.70711 ));

        assert!( n.equals( vector( 0.0, 0.70711, -0.70711 )));
    }

    #[test]
    fn normal_on_transformed_sphere(){
        let mut s = Sphere::new();
        let m = scaling( 1.0, 0.5, 1.0 ).mult( &rotation_z( PI / 5.0 ));
        s.set_transform( m );
        let n = s.normal_at( point( 0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0 ));

        assert!( n.equals( vector( 0.0, 0.97014, -0.24254 )));
    }

    #[test]
    fn sphere_has_default_material(){
        let s = Sphere::new();
        assert!( s.material.equals( material() ));
    }
}
