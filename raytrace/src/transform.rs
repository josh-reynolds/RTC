use crate::matrix::Matrix;

pub fn translation(_x: f64, _y: f64, _z: f64) -> Matrix {
    Matrix::identity()
}

#[cfg(test)]
mod tests {
    //use crate::matrix::Matrix;
    use crate::tuple::Tuple;
    use crate::number::Number;
    use crate::transform::translation;

    #[test]
    fn multiply_by_translation_matrix(){
        let t = translation(5.0, -3.0, 2.0);
        let p = Tuple::point(Number::from(-1), 
                             Number::from(4), 
                             Number::from(5));
        assert!( t.multup(&p).equals( Tuple::point(Number::from(2), 
                                                   Number::from(1), 
                                                   Number::from(7))))
    }
}
