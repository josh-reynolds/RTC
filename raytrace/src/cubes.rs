use crate::rays::Ray;
use crate::tuple::{Tuple, origin};
use crate::intersections::{Intersection, intersection, intersections};
use crate::matrix::Matrix;
use crate::shapes::{Base, Shape, shape};
use crate::materials::Material;

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
        let r2 = self.saved_ray(r);
        let sphere_to_ray = r2.origin - origin();

        let a = r2.direction.dot(&r2.direction);
        let b = 2.0 * ( r2.direction.dot(&sphere_to_ray) );
        let c = (sphere_to_ray.dot(&sphere_to_ray)) - 1.0;

        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec!();
        } else {
            let t1 = (-b - discriminant.sqrt()) / ( 2.0 * a);
            let i1 = intersection(t1, self.get_index());

            let t2 = (-b + discriminant.sqrt()) / ( 2.0 * a);
            let i2 = intersection(t2, self.get_index());

            return intersections(&[i1,i2]);
        }
    }

    fn get_index(&self) -> usize {
        self.supe.get_index()
    }

    fn set_index(&mut self, index: usize){
        self.supe.set_index(index);
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

    #[test]
    fn a_ray_intersects_a_cube(){
        let _c = cube();
    }
}
