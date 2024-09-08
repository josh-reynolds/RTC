use crate::shapes::{Shape, Base, shape};
use crate::intersections::{Intersection, intersection, intersections};
use crate::tuple::{Tuple, vector};
use crate::rays::Ray;
use crate::materials::Material;
use crate::matrix::Matrix;
use crate::equals::equals;

pub struct Cylinder {
    supe: Base,
}

impl Shape for Cylinder {
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
        
        let a = r2.direction.x.powf(2.0) + r2.direction.z.powf(2.0);

        // ray is parallel to y axis
        if equals(a, 0.0) {
            return intersections(&[]);
        }

        let b = 2.0 * r2.origin.x * r2.direction.x +
                2.0 * r2.origin.z * r2.direction.z;
        let c = r2.origin.x.powf(2.0) + r2.origin.z.powf(2.0) - 1.0;

        let disc = b.powf(2.0) - 4.0 * a * c;

        // no intersection
        if disc < 0.0 {
            return intersections(&[]);
        }

        let t0 = (-b - disc.sqrt()) / (2.0 * a);
        let t1 = (-b + disc.sqrt()) / (2.0 * a);

        return intersections(&[intersection(t0, self.get_index()),
                               intersection(t1, self.get_index())]);
    }

    fn get_index(&self) -> usize {
        self.supe.get_index()
    }

    fn set_index(&mut self, index: usize){
        self.supe.set_index(index);
    }
}

pub fn cylinder() -> Cylinder {
    Cylinder {
        supe: shape(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cylinders::cylinder;
    use crate::tuple::{point, vector};
    use crate::rays::ray;
    use crate::shapes::Shape;
    use crate::equals::equals;

    #[test]
    fn a_ray_misses_a_cylinder(){
        let cyl = cylinder();

        let direction = vector(0.0, 1.0, 0.0).normal();
        let r = ray(point(1.0, 0.0, 0.0), direction, 0);
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 0);

        let direction = vector(0.0, 1.0, 0.0).normal();
        let r = ray(point(0.0, 0.0, 0.0), direction, 0);
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 0);

        let direction = vector(1.0, 1.0, 1.0).normal();
        let r = ray(point(0.0, 0.0, -5.0), direction, 0);
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_hits_a_cylinder(){
        let cyl = cylinder();

        let direction = vector(0.0, 0.0, 1.0).normal();
        let r = ray(point(1.0, 0.0, -5.0), direction, 0);
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 2);
        assert!(equals(xs[0].t, 5.0));
        assert!(equals(xs[1].t, 5.0));

        let direction = vector(0.0, 0.0, 1.0).normal();
        let r = ray(point(0.0, 0.0, -5.0), direction, 0);
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 2);
        assert!(equals(xs[0].t, 4.0));
        assert!(equals(xs[1].t, 6.0));

        let direction = vector(0.1, 1.0, 1.0).normal();
        let r = ray(point(0.5, 0.0, -5.0), direction, 0);
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 2);
        assert!(equals(xs[0].t, 6.80798));
        assert!(equals(xs[1].t, 7.08872));
    }
}

