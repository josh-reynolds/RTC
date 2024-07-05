use crate::equals::EPSILON;
use crate::spheres::Sphere;
use crate::rays::Ray;
use crate::tuple::Tuple;

#[derive(Debug,Copy,Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere, // need to figure out type of a generic reference
                            // for now we just have Spheres
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Sphere) -> Self {
        Intersection { t, object }
    }
    
    pub fn intersections(args: &[Intersection<'a>]) -> Vec<Intersection<'a>> {
        let mut v = vec!();
        for arg in args {
            v.push(*arg);
        }
        v
    }

    pub fn hit( xs: Vec<Intersection<'a>> ) -> Option<Intersection<'_>> {
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

    pub fn equals( &self, other: Intersection<'_> ) -> bool {
        self.t == other.t && 
            self.object as *const _ == other.object as *const _
    }
}

#[derive(Debug)]
pub struct Computations<'a> {
    pub t: f64,
    pub object: &'a Sphere,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

pub fn prepare_computations( i: Intersection, r: Ray ) -> Computations {
    let mut ins = false;
    let mut n = i.object.normal_at( r.position(i.t) );
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
    use crate::intersections::{Intersection, prepare_computations};
    use crate::spheres::sphere;
    use crate::tuple::{point, vector};
    use crate::rays::ray;
    use crate::equals::equals;

    #[test]
    fn new_creates_intersections(){
        let s = sphere();
        let i = Intersection::new(3.5, &s);

        assert_eq!( 3.5, i.t );
        assert_eq!( i.object as *const _, &s as *const _ );
    }

    #[test]
    fn aggregating_intersections(){
        let s = sphere();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);

        let xs = Intersection::intersections(&[i1, i2]);
        assert_eq!( xs.len(), 2 );
        assert_eq!( xs[0].t, 1.0 );
        assert_eq!( xs[1].t, 2.0 );
    }

    #[test]
    fn hit_with_all_positive_intersections(){
        let s = sphere();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);

        let xs = Intersection::intersections(&[i2, i1]);
        let i = Intersection::hit(xs);

        assert!( i.expect("positive intersections available").equals( i1 ));
    }

    #[test]
    fn hit_when_some_intersections_are_negative(){
        let s = sphere();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);

        let xs = Intersection::intersections(&[i1, i2]);
        let i = Intersection::hit(xs);

        assert!( i.expect("positive intersection available").equals( i2 ));
    }

    #[test]
    fn hit_when_all_intersections_are_negative(){
        let s = sphere();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);

        let xs = Intersection::intersections(&[i2, i1]);
        let i = Intersection::hit(xs);

        assert!( match i {
                   Some(_x) => false,
                   None => true }
        );
    }

    #[test]
    fn hit_always_lowest_nonnegative_intersection(){
        let s = sphere();
        let i1 = Intersection::new( 5.0, &s);
        let i2 = Intersection::new( 7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new( 2.0, &s);

        let xs = Intersection::intersections(&[i1, i2, i3, i4]);
        let i = Intersection::hit(xs);

        assert!( i.expect("positive intersection available").equals( i4 ));
    }

    #[test]
    fn precomputing_intersection_state(){
        let r = ray( point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0) );
        let s = sphere();
        let i = Intersection::new(4.0, &s);

        let comps = prepare_computations(i, r);

        assert!( equals(comps.t, i.t) );
        assert_eq!( comps.object as *const _, &s as *const _ );
        assert!( comps.point.equals( point(0.0, 0.0, -1.0) ));
        assert!( comps.eyev.equals( vector(0.0, 0.0, -1.0) ));
        assert!( comps.normalv.equals( vector(0.0, 0.0, -1.0) ));
    }

    #[test]
    fn intersection_on_outside(){
        let r = ray( point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0) );
        let s = sphere();
        let i = Intersection::new(4.0, &s);

        let comps = prepare_computations(i, r);

        assert!( comps.inside == false );
    }

    #[test]
    fn intersection_on_inside(){
        let r = ray( point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0) );
        let s = sphere();
        let i = Intersection::new(1.0, &s);

        let comps = prepare_computations(i, r);

        assert!( comps.point.equals( point(0.0, 0.0, 1.0) ));
        assert!( comps.eyev.equals( vector(0.0, 0.0, -1.0) ));
        assert!( comps.normalv.equals( vector(0.0, 0.0, -1.0) ));
        assert!( comps.inside == true );
    }
}
