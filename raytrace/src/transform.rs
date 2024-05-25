use crate::matrix::Matrix;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    let mut t = Matrix::identity();
    t.set(0,3,x);
    t.set(1,3,y);
    t.set(2,3,z);
    t
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    let mut t = Matrix::identity();
    t.set(0,0,x);
    t.set(1,1,y);
    t.set(2,2,z);
    t
}

pub fn rotation_x(r: f64) -> Matrix {
    let mut t = Matrix::identity();
    t.set(1,1,  r.cos());
    t.set(1,2, -r.sin());
    t.set(2,1,  r.sin());
    t.set(2,2,  r.cos());
    t
}

#[cfg(test)]
mod tests {
    //use crate::matrix::Matrix;
    use crate::tuple::Tuple;
    use crate::number::Number;
    use crate::transform::translation;
    use crate::transform::scaling;
    use crate::transform::rotation_x;
    use std::f64::consts::PI;
    use std::f64::consts::SQRT_2;

    #[test]
    fn multiply_by_translation_matrix(){
        let t = translation(5.0, -3.0, 2.0);
        let p = Tuple::point(Number::from(-3), 
                             Number::from(4), 
                             Number::from(5));

        assert!( t.multup(&p).equals( Tuple::point(Number::from(2), 
                                                   Number::from(1), 
                                                   Number::from(7))));
    }

    #[test]
    fn multiply_by_inverse_translation_matrix(){
        let t = translation(5.0, -3.0, 2.0);
        let inv = t.inverse();
        let p = Tuple::point(Number::from(-3), 
                             Number::from(4), 
                             Number::from(5));

        assert!( inv.multup(&p).equals( Tuple::point(Number::from(-8), 
                                                     Number::from(7), 
                                                     Number::from(3))));
    }

    #[test]
    fn translation_does_not_affect_vectors(){
        let t = translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(Number::from(-3), 
                              Number::from(4), 
                              Number::from(5));

        assert!( t.multup(&v).equals( v ));
    }

    #[test]
    fn multiply_point_by_scaling_matrix(){
        let t = scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(Number::from(-4), 
                             Number::from(6), 
                             Number::from(8));

        assert!( t.multup(&p).equals( Tuple::point(Number::from(-8), 
                                                   Number::from(18), 
                                                   Number::from(32))));
    }

    #[test]
    fn multiply_vector_by_scaling_matrix(){
        let t = scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(Number::from(-4), 
                              Number::from(6), 
                              Number::from(8));

        assert!( t.multup(&v).equals( Tuple::vector(Number::from(-8), 
                                                    Number::from(18), 
                                                    Number::from(32))));
    }

    #[test]
    fn multiply_vector_by_inverse_scaling_matrix(){
        let t = scaling(2.0, 3.0, 4.0);
        let inv = t.inverse();
        let v = Tuple::vector(Number::from(-4), 
                              Number::from(6), 
                              Number::from(8));

        assert!( inv.multup(&v).equals( Tuple::vector(Number::from(-2), 
                                                      Number::from(2), 
                                                      Number::from(2))));
    }

    #[test]
    fn reflection_is_scaling_by_negative_value(){
        let t = scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(Number::from(2), 
                             Number::from(3), 
                             Number::from(4));

        assert!( t.multup(&p).equals( Tuple::point(Number::from(-2), 
                                                   Number::from(3), 
                                                   Number::from(4))));
    }

    #[test]
    fn rotate_point_around_x_axis(){
        let p = Tuple::point(Number::from(0), 
                             Number::from(1), 
                             Number::from(0));
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        assert!( half_quarter.multup(&p).equals( Tuple::point(Number::from(0), 
                                                              Number::from(SQRT_2 / 2.0), 
                                                              Number::from(SQRT_2 / 2.0))));
        assert!( full_quarter.multup(&p).equals( Tuple::point(Number::from(0), 
                                                              Number::from(0), 
                                                              Number::from(1))));
    }
}
