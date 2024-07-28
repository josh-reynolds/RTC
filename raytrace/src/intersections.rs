use crate::equals::EPSILON;
use crate::rays::Ray;
use crate::tuple::Tuple;
use crate::world::World;
//use crate::shapes::Shape;

#[derive(Debug,Copy,Clone)]
//pub struct Intersection<'a> {
pub struct Intersection {
    pub t: f64,
    //pub object: &'a Box<dyn Shape>, 
    pub object: usize,
}

//impl<'a> Intersection<'a> {
impl Intersection {
    //pub fn equals( &self, other: Intersection<'_> ) -> bool {
    pub fn equals( &self, other: Intersection ) -> bool {
        self.t == other.t && 
            //self.object as *const _ == other.object as *const _
            self.object == other.object
    }
}

//pub fn intersection<'a>(t: f64, object: &'a Box<dyn Shape>) -> Intersection<'a> {
pub fn intersection(t: f64, object: usize) -> Intersection {
    Intersection { t, object }
}

//pub fn intersections<'a>(args: &[Intersection<'a>]) -> Vec<Intersection<'a>> {
pub fn intersections(args: &[Intersection]) -> Vec<Intersection> {
    let mut v = vec!();
    for arg in args {
        v.push(*arg);
    }
    v
}

//pub fn hit<'a>( xs: Vec<Intersection<'a>> ) -> Option<Intersection<'a>> {
pub fn hit( xs: Vec<Intersection> ) -> Option<Intersection> {
    let mut lowest = xs[0];
    for i in xs {
        if (lowest.t < 0.0 || i.t < lowest.t ) && i.t >= 0.0 {
            lowest = i;
        }
    }
    if lowest.t < 0.0 {
        None
    } else {
        Some( lowest )
    }
}

#[derive(Debug)]
//pub struct Computations<'a> {
pub struct Computations {
    pub t: f64,
    //pub object: &'a Box<dyn Shape>,
    pub object: usize,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

pub fn prepare_computations( i: Intersection, r: Ray, w: &World ) -> Computations {
    let mut ins = false;
    //let mut n = i.object.normal_at( r.position(i.t) );
    let mut n = w.get_object(i.object).normal_at( r.position(i.t) );
    if n.dot( &-r.direction ) < 0.0 {
        n = -n;
        ins = true;
    }
    let op = r.position(i.t) + n * EPSILON;

    Computations { 
        t: i.t,
        object: i.object,
        point: r.position( i.t ),
        over_point: op,
        eyev: -r.direction,
        normalv: n,
        inside: ins,
    }
}

#[cfg(test)]
mod tests {
    use crate::intersections::{intersection, intersections, 
                               hit, prepare_computations};
    //use crate::spheres::sphere;
    use crate::tuple::{point, vector};
    use crate::rays::ray;
    use crate::equals::equals;
    use crate::world::default_world;
    //use crate::shapes::Shape;

    #[test]
    fn intersection_creates_intersections(){
        //let s = sphere();
        //let binding = Box::new(s) as Box<dyn Shape>;
        //let i = intersection(3.5, &binding);
        let i = intersection(3.5, 1);

        assert_eq!( 3.5, i.t );
        //assert_eq!( i.object, &binding);
        assert_eq!( i.object, 1);
    }

    #[test]
    fn aggregating_intersections(){
        //let s = sphere();
        //let binding = Box::new(s) as Box<dyn Shape>;
        //let i1 = intersection(1.0, &binding);
        //let i2 = intersection(2.0, &binding);
        let i1 = intersection(1.0, 1);
        let i2 = intersection(2.0, 2);

        let xs = intersections(&[i1, i2]);
        assert_eq!( xs.len(), 2 );
        assert_eq!( xs[0].t, 1.0 );
        assert_eq!( xs[1].t, 2.0 );
    }

    #[test]
    fn hit_with_all_positive_intersections(){
        //let s = sphere();
        //let binding = Box::new(s) as Box<dyn Shape>;
        //let i1 = intersection(1.0, &binding);
        //let i2 = intersection(2.0, &binding);
        let i1 = intersection(1.0, 1);
        let i2 = intersection(2.0, 2);

        let xs = intersections(&[i2, i1]);
        let i = hit(xs);

        assert!( i.expect("positive intersections available").equals( i1 ));
    }

    #[test]
    fn hit_when_some_intersections_are_negative(){
        //let s = sphere();
        //let binding = Box::new(s) as Box<dyn Shape>;
        //let i1 = intersection(-1.0, &binding);
        //let i2 = intersection(1.0, &binding);
        let i1 = intersection(-1.0, 1);
        let i2 = intersection(1.0, 2);

        let xs = intersections(&[i1, i2]);
        let i = hit(xs);

        assert!( i.expect("positive intersection available").equals( i2 ));
    }

    #[test]
    fn hit_when_all_intersections_are_negative(){
        //let s = sphere();
        //let binding = Box::new(s) as Box<dyn Shape>;
        //let i1 = intersection(-2.0, &binding);
        //let i2 = intersection(-1.0, &binding);
        let i1 = intersection(-2.0, 1);
        let i2 = intersection(-1.0, 2);

        let xs = intersections(&[i2, i1]);
        let i = hit(xs);

        assert!( match i {
                   Some(_x) => false,
                   None => true }
        );
    }

    #[test]
    fn hit_always_lowest_nonnegative_intersection(){
        //let s = sphere();
        //let binding = Box::new(s) as Box<dyn Shape>;
        //let i1 = intersection( 5.0, &binding);
        //let i2 = intersection( 7.0, &binding);
        //let i3 = intersection(-3.0, &binding);
        //let i4 = intersection( 2.0, &binding);
        let i1 = intersection( 5.0, 1);
        let i2 = intersection( 7.0, 2);
        let i3 = intersection(-3.0, 3);
        let i4 = intersection( 2.0, 4);

        let xs = intersections(&[i1, i2, i3, i4]);
        let i = hit(xs);

        assert!( i.expect("positive intersection available").equals( i4 ));
    }

    #[test]
    fn precomputing_intersection_state(){
        let r = ray( point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0) );
        //let s = sphere();
        //let binding = Box::new(s.clone()) as Box<dyn Shape>;
        //let i = intersection(4.0, &binding);
        let i = intersection(4.0, 1);

        let comps = prepare_computations(i, r, &default_world());

        assert!( equals(comps.t, i.t) );
        //assert_eq!( comps.object, &(Box::new(s) as Box<dyn Shape>));
        assert_eq!( comps.object, 1);
        assert!( comps.point.equals( point(0.0, 0.0, -1.0) ));
        assert!( comps.eyev.equals( vector(0.0, 0.0, -1.0) ));
        assert!( comps.normalv.equals( vector(0.0, 0.0, -1.0) ));
    }

    #[test]
    fn intersection_on_outside(){
        let r = ray( point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0) );
        //let s = sphere();
        //let binding = Box::new(s) as Box<dyn Shape>;
        //let i = intersection(4.0, &binding);
        let i = intersection(4.0, 1);

        let comps = prepare_computations(i, r, &default_world());

        assert!( comps.inside == false );
    }

    #[test]
    fn intersection_on_inside(){
        let r = ray( point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0) );
        //let s = sphere();
        //let binding = Box::new(s) as Box<dyn Shape>;
        //let i = intersection(1.0, &binding);
        let i = intersection(1.0, 1);

        let comps = prepare_computations(i, r, &default_world());

        assert!( comps.point.equals( point(0.0, 0.0, 1.0) ));
        assert!( comps.eyev.equals( vector(0.0, 0.0, -1.0) ));
        assert!( comps.normalv.equals( vector(0.0, 0.0, -1.0) ));
        assert!( comps.inside == true );
    }
}
