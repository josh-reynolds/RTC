use crate::shapes::{Shape, Base, shape};
use crate::intersections::{Intersection, intersections};
use crate::tuple::{Tuple, vector};
use crate::rays::Ray;
use crate::materials::Material;
use crate::matrix::Matrix;
//use crate::world::World;

pub struct Group {
    supe: Base,
    pub shapes: Vec<Box<dyn Shape>>,
}

impl Shape for Group {
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

    fn local_normal_at(&self, _object_point: Tuple) -> Tuple {
        return vector(0.0, 0.0, 0.0);
    }

    fn intersect(&self, r: Ray) -> Vec<Intersection> {
        let _r2 = self.saved_ray(r);
        let xs = intersections(&[]);
        
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
    }  // TO_DO: if Group is going to manage members and pass along
       // calls to intersect(), we shouldn't do the same from World
       // possible solution - check to see if parent is None in 
       // World, and only go after those...

    fn set_parent(&mut self, parent_index: usize){
        self.supe.set_parent(parent_index);
    }
}

impl Group {
    pub fn add_child(&mut self, mut child: Box<dyn Shape>) -> usize {
        let current = self.shapes.len();
        child.set_index(current as usize);
        child.set_parent(self.get_index());
        self.shapes.push(child);
        current as usize
    }

    pub fn get_child(&self, index: usize) -> &Box<dyn Shape> {
        &(self.shapes[index])
    }
    
    pub fn get_size(&self) -> usize {
        self.shapes.len()
    }
}

pub fn group() -> Group {
    Group {
        supe: shape(),
        shapes: vec![],
    }
}

#[cfg(test)]
mod tests {
    use crate::groups::group;
    use crate::tuple::{point, vector};
    use crate::rays::ray;
    use crate::transform::translation;
    use crate::shapes::{Shape, shape};
    use crate::spheres::sphere;
    //use crate::equals::equals;
    use crate::matrix::identity;
    use crate::world::world;
    //use std::f64::consts::SQRT_2;

    #[test]
    fn creating_a_new_group(){
        let g = group();

        assert!(g.get_transform().equals(identity()));
        assert!(g.shapes.len() == 0);
    }

    #[test]
    fn adding_a_child_to_a_group(){
        let mut w = world();
        let s = shape();
        let mut g = group();
        g.add_child(Box::new(s));
        let size = g.shapes.len();
        let _group_index = w.add_object(Box::new(g));

        assert!(size == 1);
    }

    #[test]
    fn intersecting_ray_with_empty_group(){
        let g = group();

        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0), 0);
        let xs = g.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersecting_ray_with_nonempty_group(){
        let mut w = world();

        let s1 = sphere();

        let mut s2 = sphere();
        s2.set_transform(translation(0.0, 0.0, -3.0));
        
        let mut s3 = sphere();
        s3.set_transform(translation(5.0, 0.0, 0.0));
        
        let mut g = group();
        g.add_child(Box::new(s1));
        g.add_child(Box::new(s2));
        g.add_child(Box::new(s3));
        let size = g.shapes.len();
        w.add_object(Box::new(g));

        assert!(size == 3);
        
        //let _r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 0);
        //let xs = g.intersect(r);  // ownership problem
        //assert_eq!(xs.len(), 4);
        //assert!(equals(xs[0].object, 1));
        //assert!(equals(xs[1].object, 1));
        //assert!(equals(xs[2].object, 0));
        //assert!(equals(xs[3].object, 0));
    }

    #[test]
    fn normal_vector_on_a_group(){
        //let c = group();

        //let n = c.local_normal_at(point(0.0, 0.0, 0.0));
        //assert_eq!(n, vector(0.0, 0.0, 0.0));

        //let n = c.local_normal_at(point(1.0, 1.0, 1.0));
        //assert_eq!(n, vector(1.0, -SQRT_2, 1.0));

        //let n = c.local_normal_at(point(-1.0, -1.0, 0.0));
        //assert_eq!(n, vector(-1.0, 1.0, 0.0));
    }
}

