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
}
