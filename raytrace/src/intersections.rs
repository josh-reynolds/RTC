use crate::equals::EPSILON;
use crate::rays::Ray;
use crate::tuple::Tuple;
use crate::world::World;

#[derive(Debug,Copy,Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: usize,
}

impl Intersection {
    pub fn equals( &self, other: Intersection ) -> bool {
        self.t == other.t && 
        self.object == other.object
    }
}

pub fn intersection(t: f64, object: usize) -> Intersection {
    Intersection { t, object }
}

pub fn intersections(args: &[Intersection]) -> Vec<Intersection> {
    let mut v = vec!();
    for arg in args {
        v.push(*arg);
    }
    v
}

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

pub fn schlick(comps: Computations) -> f64 {
    let mut cos = comps.eyev.dot(&comps.normalv);

    // total internal reflection case
    if comps.n1 > comps.n2 {
        let n = comps.n1 / comps.n2;
        let sin2_t = n.powf(2.0) * (1.0 - cos.powf(2.0));
        if sin2_t > 1.0 {
            return 1.0 
        }

        cos = (1.0 - sin2_t).sqrt();
    }

    let r0 = ((comps.n1 - comps.n2) / (comps.n1 + comps.n2)).powf(2.0);
    r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
}

#[derive(Debug)]
pub struct Computations {
    pub t: f64,
    pub object: usize,
    pub point: Tuple,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub reflectv: Tuple,
    pub count: usize,
    pub n1: f64,   // refractive index coming FROM
    pub n2: f64,   // refractive index going TO
}

pub fn prepare_computations( hit: Intersection, 
                             r: Ray, 
                             w: &World, 
                             xs: &Vec<Intersection> ) -> Computations {
    let mut ins = false;
    let mut n = w.get_object(hit.object).normal_at( r.position(hit.t) );
    if n.dot( &-r.direction ) < 0.0 {
        n = -n;
        ins = true;
    }
    let op = r.position(hit.t) + n * EPSILON;
    let up = r.position(hit.t) - n * EPSILON;
    let rv = r.direction.reflect(&n);

    let mut containers: Vec<usize> = vec!();
    let mut n1 = 1.0;
    let mut n2 = 1.0;

    for i in xs {
        if i.equals(hit) {
            if containers.len() == 0 {
                n1 = 1.0;
            } else {
                let last = containers[containers.len()-1];
                n1 = w.get_object(last).get_material().refractive_index;
            }
        }

        if containers.contains(&i.object) {
            containers.retain(|x| *x != i.object);
        } else {
            containers.push(i.object);
        }

        if i.equals(hit) {
            if containers.len() == 0 {
                n2 = 1.0;
            } else {
                let last = containers[containers.len()-1];
                n2 = w.get_object(last).get_material().refractive_index;
            }
            break;
        }
    }

    Computations { 
        t: hit.t,
        object: hit.object,
        point: r.position( hit.t ),
        over_point: op,
        under_point: up,
        eyev: -r.direction,
        normalv: n,
        inside: ins,
        reflectv: rv,
        count: r.count,
        n1: n1,
        n2: n2,
    }
}

#[cfg(test)]
mod tests {
    use crate::intersections::{intersection, intersections, 
                               hit, prepare_computations, schlick};
    use crate::tuple::{point, vector};
    use crate::rays::ray;
    use crate::equals::equals;
    use crate::world::{world, default_world};
    use crate::planes::plane;
    use crate::spheres::glass_sphere;
    use crate::shapes::Shape;
    use crate::transform::{scaling, translation};
    use crate::materials::material;
    use crate::equals::EPSILON;
    use std::f64::consts::SQRT_2;

    #[test]
    fn intersection_creates_intersections(){
        let i = intersection(3.5, 1);

        assert_eq!( 3.5, i.t );
        assert_eq!( i.object, 1);
    }

    #[test]
    fn aggregating_intersections(){
        let i1 = intersection(1.0, 1);
        let i2 = intersection(2.0, 2);

        let xs = intersections(&[i1, i2]);
        assert_eq!( xs.len(), 2 );
        assert_eq!( xs[0].t, 1.0 );
        assert_eq!( xs[1].t, 2.0 );
    }

    #[test]
    fn hit_with_all_positive_intersections(){
        let i1 = intersection(1.0, 1);
        let i2 = intersection(2.0, 2);

        let xs = intersections(&[i2, i1]);
        let i = hit(xs);

        assert!( i.expect("positive intersections available").equals( i1 ));
    }

    #[test]
    fn hit_when_some_intersections_are_negative(){
        let i1 = intersection(-1.0, 1);
        let i2 = intersection(1.0, 2);

        let xs = intersections(&[i1, i2]);
        let i = hit(xs);

        assert!( i.expect("positive intersection available").equals( i2 ));
    }

    #[test]
    fn hit_when_all_intersections_are_negative(){
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
        let r = ray( point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 0 );
        let i = intersection(4.0, 1);
        let xs = intersections(&[i]);

        let comps = prepare_computations(i, r, &default_world(), &xs);

        assert!( equals(comps.t, i.t) );
        assert_eq!( comps.object, 1);
        assert!( comps.point.equals( point(0.0, 0.0, -1.0) ));
        assert!( comps.eyev.equals( vector(0.0, 0.0, -1.0) ));
        assert!( comps.normalv.equals( vector(0.0, 0.0, -1.0) ));
    }

    #[test]
    fn intersection_on_outside(){
        let r = ray( point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 0 );
        let i = intersection(4.0, 1);
        let xs = intersections(&[i]);

        let comps = prepare_computations(i, r, &default_world(), &xs);

        assert!( comps.inside == false );
    }

    #[test]
    fn intersection_on_inside(){
        let r = ray( point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0), 0 );
        let i = intersection(1.0, 1);
        let xs = intersections(&[i]);

        let comps = prepare_computations(i, r, &default_world(), &xs);

        assert!( comps.point.equals( point(0.0, 0.0, 1.0) ));
        assert!( comps.eyev.equals( vector(0.0, 0.0, -1.0) ));
        assert!( comps.normalv.equals( vector(0.0, 0.0, -1.0) ));
        assert!( comps.inside == true );
    }

    #[test]
    fn precomputing_reflection_vector(){
        let mut w = world();
        let p = plane();
        w.add_object(Box::new(p));
        let r = ray( point(0.0, 1.0, -1.0), vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0), 0 );
        let i = intersection(SQRT_2, 0);
        let xs = intersections(&[i]);

        let comps = prepare_computations(i, r, &w, &xs);

        assert!{ comps.reflectv.equals( vector(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0) )};
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections(){
        let mut w = world();

        let mut a = glass_sphere();
        a.set_transform(scaling(2.0, 2.0, 2.0));
        let mut mat = material();
        mat.refractive_index = 1.5;
        a.set_material(mat);
        w.add_object(Box::new(a));

        let mut b = glass_sphere();
        b.set_transform(translation(0.0, 0.0, -0.25));
        let mut mat = material();
        mat.refractive_index = 2.0;
        b.set_material(mat);
        w.add_object(Box::new(b));

        let mut c = glass_sphere();
        c.set_transform(translation(0.0, 0.0, 0.25));
        let mut mat = material();
        mat.refractive_index = 2.5;
        c.set_material(mat);
        w.add_object(Box::new(c));
        
        let r = ray(point(0.0, 0.0, -4.0), vector(0.0, 0.0, 1.0), 0);

        let i1 = intersection(2.00, 0); // more 'correct' to use a.get_index()
        let i2 = intersection(2.75, 1); // here, but that will cause a borrowing
        let i3 = intersection(3.25, 2); // issue and require clone() above, 
        let i4 = intersection(4.75, 1); // simpler to have this index hardcoded
        let i5 = intersection(5.25, 2); // for the purposes of this test
        let i6 = intersection(6.00, 0);
        let xs = intersections(&[i1, i2, i3, i4, i5, i6]);

        let comps = prepare_computations(xs[0], r, &w, &xs);
        assert_eq!(comps.n1, 1.0);
        assert_eq!(comps.n2, 1.5);
        
        let comps = prepare_computations(xs[1], r, &w, &xs);
        assert_eq!(comps.n1, 1.5);
        assert_eq!(comps.n2, 2.0);

        let comps = prepare_computations(xs[2], r, &w, &xs);
        assert_eq!(comps.n1, 2.0);
        assert_eq!(comps.n2, 2.5);

        let comps = prepare_computations(xs[3], r, &w, &xs);
        assert_eq!(comps.n1, 2.5);
        assert_eq!(comps.n2, 2.5);

        let comps = prepare_computations(xs[4], r, &w, &xs);
        assert_eq!(comps.n1, 2.5);
        assert_eq!(comps.n2, 1.5);

        let comps = prepare_computations(xs[5], r, &w, &xs);
        assert_eq!(comps.n1, 1.5);
        assert_eq!(comps.n2, 1.0);
    }

    #[test]
    fn under_point_offset_below_surface(){
        let mut w = world();
        
        let mut shape = glass_sphere();
        shape.set_transform(translation(0.0, 0.0, 1.0));
        w.add_object(Box::new(shape));

        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 0);
        let i = intersection(5.0, 0); 
        let xs = intersections(&[i]);

        let comps = prepare_computations(i, r, &w, &xs);

        assert!(comps.under_point.z > EPSILON/2.0);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn schlick_under_total_internal_reflection(){
        let mut w = world();

        let shape = glass_sphere();
        w.add_object(Box::new(shape));

        let r = ray(point(0.0, 0.0, SQRT_2/2.0), vector(0.0, 1.0, 0.0), 0);
        let i1 = intersection(-SQRT_2/2.0, 0); 
        let i2 = intersection( SQRT_2/2.0, 0); 
        let xs = intersections(&[i1, i2]);
        let comps = prepare_computations(xs[1], r, &w, &xs);

        let reflectance = schlick(comps);

        assert!(reflectance == 1.0);
    }

    #[test]
    fn schlick_with_perpendicular_view_angle(){
        let mut w = world();

        let shape = glass_sphere();
        w.add_object(Box::new(shape));

        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0), 0);
        let i1 = intersection(-1.0, 0); 
        let i2 = intersection( 1.0, 0); 
        let xs = intersections(&[i1, i2]);
        let comps = prepare_computations(xs[1], r, &w, &xs);

        let reflectance = schlick(comps);

        assert!(equals(reflectance, 0.04));
    }

    #[test]
    fn schlick_with_small_view_angle(){
        let mut w = world();

        let shape = glass_sphere();
        w.add_object(Box::new(shape));

        let r = ray(point(0.0, 0.99, -2.0), vector(0.0, 0.0, 1.0), 0);
        let i1 = intersection(1.8589, 0); 
        let xs = intersections(&[i1]);
        let comps = prepare_computations(xs[0], r, &w, &xs);

        let reflectance = schlick(comps);

        assert!(equals(reflectance, 0.48873));
    }
}
