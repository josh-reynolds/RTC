use crate::shapes::{Shape, Base, shape};
use crate::shape_index::ShapeIndex;
use crate::intersections::{Intersection, intersection, intersections};
use crate::tuple::{Tuple, vector};
use crate::rays::Ray;
use crate::materials::Material;
use crate::matrix::Matrix;
use crate::equals::equals;
use crate::equals::EPSILON;
use std::f64::INFINITY;

pub struct Cone {
    supe: Base,
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
}

impl Shape for Cone {
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

    fn local_normal_at(&self, object_point: Tuple) -> Tuple {
        let dist = object_point.x.powf(2.0) + object_point.z.powf(2.0);
        let mut y = dist.sqrt();
        if object_point.y > 0.0 {
            y = -y;
        }

        if dist < 1.0 && object_point.y >= self.maximum - EPSILON {
            return vector(0.0, 1.0, 0.0);
        } else if dist < 1.0 && object_point.y <= self.minimum + EPSILON {
            return vector(0.0, -1.0, 0.0);
        } else {
            return vector(object_point.x, y, object_point.z);
        }
    }

    fn intersect(&self, r: Ray) -> Vec<Intersection> {
        let r2 = self.saved_ray(r);
        let mut xs = intersections(&[]);
        
        // formulae from (RTC p. 189)
        
        let a = r2.direction.x.powf(2.0) - 
                r2.direction.y.powf(2.0) + 
                r2.direction.z.powf(2.0);

        let b = 2.0 * r2.origin.x * r2.direction.x -
                2.0 * r2.origin.y * r2.direction.y +
                2.0 * r2.origin.z * r2.direction.z;
        
        let c = r2.origin.x.powf(2.0) -
                r2.origin.y.powf(2.0) + 
                r2.origin.z.powf(2.0);

        // ray is parallel to side of cone
        if equals(a, 0.0) {
            if equals(b, 0.0) {
              // misses side
              xs = self.intersect_caps(r2, xs); 
              return xs;
            } else {
              // hits other half
              let t = -c / (2.0 * b);
              xs.push(intersection(t, self.get_index()));
              xs = self.intersect_caps(r2, xs); 
              return xs;
            }
        }

        let disc = b.powf(2.0) - 4.0 * a * c;

        // no intersection
        if disc < 0.0 {
            return xs;
        }

        let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
        let mut t1 = (-b + disc.sqrt()) / (2.0 * a);
        if t0 > t1 {
            let temp = t1;
            t1 = t0;
            t0 = temp;
        }

        let y0 = r2.origin.y + t0 * r2.direction.y;
        if self.minimum < y0 && y0 < self.maximum {
            xs.push(intersection(t0, self.get_index()));
        }

        let y1 = r2.origin.y + t1 * r2.direction.y;
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(intersection(t1, self.get_index()));
        }
        
        xs = self.intersect_caps(r2, xs);
        return xs;
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

    fn get_object(&self, _index: usize) -> &Box<dyn Shape> {
        &(Box::new(shape()) as Box<dyn Shape>)
    }

    fn get_size(&self) -> usize {
        0
    }
}

impl Cone {
    fn intersect_caps(&self, r: Ray, mut xs: Vec<Intersection>) -> Vec<Intersection> {
        if !self.closed || equals(r.direction.y, 0.0) {
            return xs
        }

        // check lower end cap
        let t = (self.minimum - r.origin.y) / r.direction.y;
        if self.check_cap(r, t, self.minimum){
            xs.push(intersection(t, self.get_index()));
        }

        // check upper end cap
        let t = (self.maximum - r.origin.y) / r.direction.y;
        if self.check_cap(r, t, self.maximum){
            xs.push(intersection(t, self.get_index()));
        }

        return xs
    }

    fn check_cap(&self, r: Ray, t: f64, radius: f64) -> bool {
        let x = r.origin.x + t * r.direction.x;
        let z = r.origin.z + t * r.direction.z;
        (x.powf(2.0) + z.powf(2.0)) <= radius.abs()
    }
}

pub fn cone() -> Cone {
    Cone {
        supe: shape(),
        minimum: -INFINITY,
        maximum:  INFINITY,
        closed: false,
    }
}

#[cfg(test)]
mod tests {
    use crate::cones::cone;
    use crate::tuple::{point, vector};
    use crate::rays::ray;
    use crate::shapes::Shape;
    use crate::equals::equals;
    use std::f64::INFINITY;
    use std::f64::consts::SQRT_2;

    #[test]
    fn a_ray_misses_a_cone(){
        let c = cone();
        
        let direction = vector(1.0, 0.0, 0.0).normal();
        let r = ray(point(1.0, 0.0, 1.0), direction, 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_hits_a_cone(){
        let c = cone();

        let direction = vector(0.0, 0.0, 1.0).normal();
        let r = ray(point(0.0, 0.0, -5.0), direction, 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 2);
        assert!(equals(xs[0].t, 5.0));
        assert!(equals(xs[1].t, 5.0));

        let direction = vector(1.0, 1.0, 1.0).normal();
        let r = ray(point(0.0, 0.0, -5.0), direction, 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 2);
        assert!(equals(xs[0].t, 8.66025));
        assert!(equals(xs[1].t, 8.66025));

        let direction = vector(-0.5, -1.0, 1.0).normal();
        let r = ray(point(1.0, 1.0, -5.0), direction, 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 2);
        assert!(equals(xs[0].t,  4.55006));
        assert!(equals(xs[1].t, 49.44994));
    }

    #[test]
    fn ray_parallel_to_side_hits_a_cone(){
        let c = cone();

        let direction = vector(0.0, 1.0, 1.0).normal();
        let r = ray(point(0.0, 0.0, -1.0), direction, 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 1);
        assert!(equals(xs[0].t, 0.35355));
    }

    #[test]
    fn normal_vector_on_a_cone(){
        let c = cone();

        let n = c.local_normal_at(point(0.0, 0.0, 0.0));
        assert_eq!(n, vector(0.0, 0.0, 0.0));

        let n = c.local_normal_at(point(1.0, 1.0, 1.0));
        assert_eq!(n, vector(1.0, -SQRT_2, 1.0));

        let n = c.local_normal_at(point(-1.0, -1.0, 0.0));
        assert_eq!(n, vector(-1.0, 1.0, 0.0));
    }

    #[test]
    fn default_min_max_for_cone(){
        let c = cone();

        assert_eq!(c.minimum, -INFINITY);
        assert_eq!(c.maximum,  INFINITY);
    }

    // needs update? - uncertain, test data comes from
    // cylinder and has not been reworked, but the test
    // is passing as-is. Scrub at some point.
    #[test]
    fn intersecting_a_truncated_cone(){
        let mut c = cone();
        c.minimum = 1.0;
        c.maximum = 2.0;

        let direction = vector(0.1, 1.0, 0.0).normal();
        let r = ray(point(0.0, 1.5, 0.0), direction, 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 0);

        let direction = vector(0.0, 0.0, 1.0).normal();
        let r = ray(point(0.0, 3.0, -5.0), direction, 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 0);

        let direction = vector(0.0, 0.0, 1.0).normal();
        let r = ray(point(0.0, 0.0, -5.0), direction, 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 0);

        let direction = vector(0.0, 0.0, 1.0).normal();
        let r = ray(point(0.0, 2.0, -5.0), direction, 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 0);

        let direction = vector(0.0, 0.0, 1.0).normal();
        let r = ray(point(0.0, 1.0, -5.0), direction, 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 0);

        let direction = vector(0.0, 0.0, 1.0).normal();
        let r = ray(point(0.0, 1.5, -2.0), direction, 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn default_closed_value_for_cones(){
        let c= cone();

        assert_eq!(c.closed, false);
    }

    #[test]
    fn intersecting_caps_of_closed_cone(){
        let mut c = cone();
        c.minimum = -0.5;
        c.maximum =  0.5;
        c.closed = true;

        let direction = vector(0.0, 1.0, 0.0).normal();
        let r = ray(point(0.0, 0.0, -5.0), direction, 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 0);

        let direction = vector(0.0, 1.0, 1.0).normal();
        let r = ray(point(0.0, 0.0, -0.25), direction, 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 2);

        let direction = vector(0.0, 1.0, 0.0).normal();
        let r = ray(point(0.0, 0.0, -0.25), direction, 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 4);
    }

    #[test]
    fn normal_vector_on_cone_end_caps(){
        let mut c = cone();
        c.minimum = 1.0;
        c.maximum = 2.0;
        c.closed = true;

        let n = c.local_normal_at(point(0.0, 1.0, 0.0));
        assert!(n == vector(0.0, -1.0, 0.0));

        let n = c.local_normal_at(point(0.5, 1.0, 0.0));
        assert!(n == vector(0.0, -1.0, 0.0));

        let n = c.local_normal_at(point(0.0, 1.0, 0.5));
        assert!(n == vector(0.0, -1.0, 0.0));

        let n = c.local_normal_at(point(0.0, 2.0, 0.0));
        assert!(n == vector(0.0, 1.0, 0.0));

        let n = c.local_normal_at(point(0.5, 2.0, 0.0));
        assert!(n == vector(0.0, 1.0, 0.0));

        let n = c.local_normal_at(point(0.0, 2.0, 0.5));
        assert!(n == vector(0.0, 1.0, 0.0));
    }
}

