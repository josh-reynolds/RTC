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

#[cfg(test)]
mod tests {
    //use crate::matrix::Matrix;
    use crate::tuple::Tuple;
    use crate::number::Number;
    use crate::transform::translation;
    use crate::transform::scaling;

    #[test]
    fn multiply_by_translation_matrix(){
        let t = translation(5.0, -3.0, 2.0);
        let p = Tuple::point(Number::from(-3), 
                             Number::from(4), 
                             Number::from(5));

        assert!( t.multup(&p).equals( Tuple::point(Number::from(2), 
                                                   Number::from(1), 
                                                   Number::from(7))))
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
                                                     Number::from(3))))
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
                                                   Number::from(32))))
    }

    #[test]
    fn multiply_vector_by_scaling_matrix(){
        let t = scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(Number::from(-4), 
                              Number::from(6), 
                              Number::from(8));

        assert!( t.multup(&v).equals( Tuple::vector(Number::from(-8), 
                                                    Number::from(18), 
                                                    Number::from(32))))
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
                                                      Number::from(2))))
    }
}
