use crate::spheres::Sphere;

#[derive(Debug,Copy,Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere, // need to figure out type of a generic reference
                            // for now we just have Spheres
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Sphere) -> Self {
        Intersection { t: t, object: object }
    }
    
    pub fn intersections(i1: Intersection<'a>, i2: Intersection<'a>) -> Vec<Intersection<'a>> {
        vec!(i1, i2)
    }

    pub fn hit( xs: Vec<Intersection<'a>> ) -> Self {
        let mut lowest = xs[0];
        for i in xs {
            if i.t < lowest.t {
                lowest = i;
            }
        }
        lowest
    }

    pub fn equals( &self, other: Intersection<'_> ) -> bool {
        self.t == other.t && 
            self.object as *const _ == other.object as *const _
    }
}

#[cfg(test)]
mod tests {
    use crate::intersections::Intersection;
    use crate::spheres::Sphere;

    #[test]
    fn new_creates_intersections(){
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);

        assert_eq!( 3.5, i.t );
        assert_eq!( i.object as *const _, &s as *const _ );
    }

    #[test]
    fn aggregating_intersections(){
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);

        let xs = Intersection::intersections(i1, i2);
        assert_eq!( xs.len(), 2 );
        assert_eq!( xs[0].t, 1.0 );
        assert_eq!( xs[1].t, 2.0 );
    }

    #[test]
    fn hit_with_all_positive_intersections(){
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);

        let xs = Intersection::intersections(i2, i1);
        let i = Intersection::hit(xs);

        assert!( i.equals( i1 ));
    }
}
