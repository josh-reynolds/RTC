use crate::rays::Ray;
use crate::tuple::{Tuple, vector};
use crate::intersections::{Intersection, intersection, intersections};
use crate::matrix::Matrix;
use crate::shapes::{Base, Shape, shape};
use crate::shape_index::ShapeIndex;
use crate::materials::Material;
use crate::equals::EPSILON;
use std::f64::INFINITY;

pub struct Cube {
    supe: Base,
}

impl Shape for Cube {
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
        let maxs = [object_point.x.abs(), object_point.y.abs(), object_point.z.abs()];
        let maxc = maxs.iter().max_by(|a,b| a.total_cmp(b)).unwrap();

        if *maxc == object_point.x.abs() {
            return vector(object_point.x, 0.0, 0.0);
        } else if *maxc == object_point.y.abs() {
            return vector(0.0, object_point.y, 0.0);
        }
        return vector(0.0, 0.0, object_point.z);
    }

    fn intersect(&self, r: Ray) -> Vec<Intersection> {
        let r2 = self.saved_ray(r);
        
        let xvals = self.check_axis(r2.origin.x, r2.direction.x);
        let yvals = self.check_axis(r2.origin.y, r2.direction.y);
        let zvals = self.check_axis(r2.origin.z, r2.direction.z);

        let mins = [xvals.0, yvals.0, zvals.0];
        let maxs = [xvals.1, yvals.1, zvals.1];
        
        let tmin = mins.iter().max_by(|a,b| a.total_cmp(b)).unwrap();
        let tmax = maxs.iter().min_by(|a,b| a.total_cmp(b)).unwrap();

        if tmin > tmax {
            return intersections(&[]);
        }

        let i1 = intersection(*tmin, self.get_index());
        let i2 = intersection(*tmax, self.get_index());
        return intersections(&[i1, i2]);
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

impl Cube {
    fn check_axis(&self, o: f64, d: f64) -> (f64, f64){
        let tmin_numerator = -1.0 - o;
        let tmax_numerator =  1.0 - o;

        let mut tmin;
        let mut tmax;

        if d.abs() > EPSILON {
            tmin = tmin_numerator / d;
            tmax = tmax_numerator / d;
        } else {
            tmin = tmin_numerator * INFINITY;
            tmax = tmax_numerator * INFINITY;
        }

        if tmin > tmax {
            let temp = tmin;
            tmin = tmax;
            tmax = temp;
        }

        (tmin, tmax)
    }
}

pub fn cube() -> Cube {
    Cube {
        supe: shape(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cubes::cube;
    use crate::shapes::Shape;
    use crate::tuple::{point, vector};
    use crate::rays::ray;

    #[test]
    fn a_ray_intersects_a_cube(){
        let c = cube();

        // positive x
        let r = ray(point(5.0, 0.5, 0.0), vector(-1.0, 0.0, 0.0), 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);

        // negative x
        let r = ray(point(-5.0, 0.5, 0.0), vector(1.0, 0.0, 0.0), 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);

        // positive y
        let r = ray(point(0.5, 5.0, 0.0), vector(0.0, -1.0, 0.0), 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
        
        // negative y
        let r = ray(point(0.5, -5.0, 0.0), vector(0.0, 1.0, 0.0), 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
        
        // positive z
        let r = ray(point(0.5, 0.0, 5.0), vector(0.0, 0.0, -1.0), 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
        
        // negative z
        let r = ray(point(0.5, 0.0, -5.0), vector(0.0, 0.0, 1.0), 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);

        // inside
        let r = ray(point(0.0, 0.5, 0.0), vector(0.0, 0.0, 1.0), 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn a_ray_misses_a_cube(){
        let c = cube();

        let r = ray(point(-2.0, 0.0, 0.0), vector(0.2673, 0.5345, 0.8018), 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 0);

        let r = ray(point(0.0, -2.0, 0.0), vector(0.8018, 0.2673, 0.5345), 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 0);

        let r = ray(point(0.0, 0.0, -2.0), vector(0.5345, 0.8018, 0.2673), 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 0);

        let r = ray(point(2.0, 0.0, 2.0), vector(0.0, 0.0, -1.0), 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 0);

        let r = ray(point(0.0, 2.0, 2.0), vector(0.0, -1.0, 0.0), 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 0);

        let r = ray(point(2.0, 2.0, 0.0), vector(-1.0, 0.0, 0.0), 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn normal_on_surface_of_cube(){
        let c = cube();

        let p = point(1.0, 0.5, -0.8);
        let normal = c.local_normal_at(p);
        assert!(normal.equals(vector(1.0, 0.0, 0.0)));

        let p = point(-1.0, -0.2, 0.9);
        let normal = c.local_normal_at(p);
        assert!(normal.equals(vector(-1.0, 0.0, 0.0)));

        let p = point(-0.4, 1.0, -0.1);
        let normal = c.local_normal_at(p);
        assert!(normal.equals(vector(0.0, 1.0, 0.0)));

        let p = point(0.3, -1.0, -0.7);
        let normal = c.local_normal_at(p);
        assert!(normal.equals(vector(0.0, -1.0, 0.0)));

        let p = point(-0.6, 0.3, 1.0);
        let normal = c.local_normal_at(p);
        assert!(normal.equals(vector(0.0, 0.0, 1.0)));

        let p = point(0.4, 0.4, -1.0);
        let normal = c.local_normal_at(p);
        assert!(normal.equals(vector(0.0, 0.0, -1.0)));

        let p = point(1.0, 1.0, 1.0);
        let normal = c.local_normal_at(p);
        assert!(normal.equals(vector(1.0, 0.0, 0.0)));

        let p = point(-1.0, -1.0, -1.0);
        let normal = c.local_normal_at(p);
        assert!(normal.equals(vector(-1.0, 0.0, 0.0)));
    }
}
