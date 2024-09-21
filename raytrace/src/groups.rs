use crate::shapes::{Shape, Base, shape};
use crate::intersections::{Intersection, intersections};
use crate::tuple::{Tuple, vector};
use crate::rays::Ray;
use crate::materials::Material;
use crate::matrix::Matrix;
use crate::world::World;

pub struct Group {
    supe: Base,
    pub shapes: Vec<usize>,
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
    pub fn add_child(&mut self, child_index: usize, world: &World){
        self.shapes.push(child_index);
        //world.get_object(child_index).set_parent(self.get_index());
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
    use crate::shapes::{Shape, shape};
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
        let shape_index = w.add_object(Box::new(s));

        let mut g = group();
        g.add_child(shape_index, &w);
        let sz = g.get_size();
        let group_index = w.add_object(Box::new(g));

        assert!(sz == 1);
        //println!("{:?}", w.get_object(group_index).get_transform());
        //println!("{:?}", w.get_object(shape_index).get_parent());
        //println!("{:?}", w);
        //assert!(false);
        //assert_eq!(w.get_object(group_index).get_child(0).get_index(),
                   //shape_index);
        assert!(w.get_object(shape_index).get_parent() == Some(group_index));
    }

    #[test]
    fn a_ray_misses_a_group(){
        let c = group();
        
        let direction = vector(1.0, 0.0, 0.0).normal();
        let r = ray(point(1.0, 0.0, 1.0), direction, 0);
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_hits_a_group(){
        //let c = group();

        //let direction = vector(0.0, 0.0, 1.0).normal();
        //let r = ray(point(0.0, 0.0, -5.0), direction, 0);
        //let xs = c.intersect(r);
        //assert_eq!(xs.len(), 2);
        //assert!(equals(xs[0].t, 5.0));
        //assert!(equals(xs[1].t, 5.0));

        //let direction = vector(1.0, 1.0, 1.0).normal();
        //let r = ray(point(0.0, 0.0, -5.0), direction, 0);
        //let xs = c.intersect(r);
        //assert_eq!(xs.len(), 2);
        //assert!(equals(xs[0].t, 8.66025));
        //assert!(equals(xs[1].t, 8.66025));

        //let direction = vector(-0.5, -1.0, 1.0).normal();
        //let r = ray(point(1.0, 1.0, -5.0), direction, 0);
        //let xs = c.intersect(r);
        //assert_eq!(xs.len(), 2);
        //assert!(equals(xs[0].t,  4.55006));
        //assert!(equals(xs[1].t, 49.44994));
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
