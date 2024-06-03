use crate::spheres::Sphere;

#[derive(Debug)]
pub struct Intersection<'a> {
    t: f64,
    object: &'a Sphere, // need to figure out type of a generic reference
                        // for now we just have Spheres
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Sphere) -> Self {
        Intersection { t: t, object: object }
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
}
