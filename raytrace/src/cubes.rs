use crate::rays::Ray;
use crate::tuple::{Tuple, origin};
use crate::intersections::{Intersection, intersection, intersections};
use crate::matrix::Matrix;
use crate::shapes::{Base, Shape, shape};
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
        object_point - origin()
    }

    fn intersect(&self, r: Ray) -> Vec<Intersection> {
        let xvals = self.check_axis(r.origin.x, r.direction.x);
        let yvals = self.check_axis(r.origin.y, r.direction.y);
        let zvals = self.check_axis(r.origin.z, r.direction.z);

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
}
