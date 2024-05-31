use crate::tuple::Tuple;

#[derive(Debug)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin: origin, direction: direction }
    }

    pub fn position(&self, distance: f64) -> Tuple {
        self.origin.plus( &self.direction.mult( distance ))
    }
}

#[cfg(test)]
mod tests {
    use crate::rays::Ray;
    use crate::tuple::Tuple;
    use crate::number::Number;

    #[test]
    fn new_creates_rays(){
        let o = Tuple::point( Number::from(1),
                              Number::from(2),
                              Number::from(3));
        let d = Tuple::vector( Number::from(4),
                               Number::from(5),
                               Number::from(6));

        let r = Ray::new(o, d);
        assert!( r.origin.equals(Tuple::point( Number::from(1),
                                               Number::from(2),
                                               Number::from(3))) );
        assert!( r.direction.equals(Tuple::vector( Number::from(4),
                                                   Number::from(5),
                                                   Number::from(6))) );
    }

    #[test]
    fn computing_point_from_distance(){
        let o = Tuple::point( Number::from(2),
                              Number::from(3),
                              Number::from(4));
        let d = Tuple::vector( Number::from(1),
                               Number::from(0),
                               Number::from(0));

        let r = Ray::new(o, d);
        assert!( r.position(0.0).equals(Tuple::point( Number::from(2),
                                                      Number::from(3),
                                                      Number::from(4))) );
        assert!( r.position(1.0).equals(Tuple::point( Number::from(3),
                                                      Number::from(3),
                                                      Number::from(4))) );
        assert!( r.position(-1.0).equals(Tuple::point( Number::from(1),
                                                       Number::from(3),
                                                       Number::from(4))) );
        assert!( r.position(2.5).equals(Tuple::point( Number::from(4.5),
                                                      Number::from(3),
                                                      Number::from(4))) );

    }
}
