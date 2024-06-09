use crate::tuple::Tuple;
use crate::matrix::Matrix;

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

    pub fn transform(&self, t: Matrix) -> Self {
        Ray::new( t.multup( &self.origin ),
                  t.multup( &self.direction ))
    }
}

#[cfg(test)]
mod tests {
    use crate::rays::Ray;
    use crate::tuple::Tuple;
    use crate::number::Number;
    use crate::transform::translation;
    use crate::transform::scaling;

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

    #[test]
    fn translating_a_ray(){
        let o = Tuple::point( Number::from(1),
                              Number::from(2),
                              Number::from(3));
        let d = Tuple::vector( Number::from(0),
                               Number::from(1),
                               Number::from(0));
        let r = Ray::new(o, d);

        let m = translation(3.0, 4.0, 5.0);
        let r2 = r.transform(m);

        assert!( r2.origin.equals(Tuple::point( Number::from(4),
                                                Number::from(6),
                                                Number::from(8))) );
        assert!( r2.direction.equals(Tuple::vector( Number::from(0),
                                                    Number::from(1),
                                                    Number::from(0))) );
    }

    #[test]
    fn scaling_a_ray(){
        let o = Tuple::point( Number::from(1),
                              Number::from(2),
                              Number::from(3));
        let d = Tuple::vector( Number::from(0),
                               Number::from(1),
                               Number::from(0));
        let r = Ray::new(o, d);

        let m = scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(m);

        assert!( r2.origin.equals(Tuple::point( Number::from(2),
                                                Number::from(6),
                                                Number::from(12))) );
        assert!( r2.direction.equals(Tuple::vector( Number::from(0),
                                                    Number::from(3),
                                                    Number::from(0))) );
    }
}
