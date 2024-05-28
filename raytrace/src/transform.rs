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

pub fn rotation_y(r: f64) -> Matrix {
    let mut t = Matrix::identity();
    t.set(0,0,  r.cos());
    t.set(0,2,  r.sin());
    t.set(2,0, -r.sin());
    t.set(2,2,  r.cos());
    t
}

pub fn rotation_z(r: f64) -> Matrix {
    let mut t = Matrix::identity();
    t.set(0,0,  r.cos());
    t.set(0,1, -r.sin());
    t.set(1,0,  r.sin());
    t.set(1,1,  r.cos());
    t
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    let mut t = Matrix::identity();
    t.set(0,1, xy);
    t.set(0,2, xz);
    t.set(1,0, yx);
    t.set(1,2, yz);
    t.set(2,0, zx);
    t.set(2,1, zy);
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
    use crate::transform::rotation_y;
    use crate::transform::rotation_z;
    use crate::transform::shearing;
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

    #[test]
    fn inverse_of_rotation_rotates_opposite_direction(){
        let p = Tuple::point(Number::from(0), 
                             Number::from(1), 
                             Number::from(0));
        let half_quarter = rotation_x(PI / 4.0);
        let inv = half_quarter.inverse();

        assert!( inv.multup(&p).equals( Tuple::point(Number::from(0), 
                                                     Number::from( SQRT_2 / 2.0), 
                                                     Number::from(-SQRT_2 / 2.0))));
    }

    #[test]
    fn rotate_point_around_y_axis(){
        let p = Tuple::point(Number::from(0), 
                             Number::from(0), 
                             Number::from(1));
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);

        assert!( half_quarter.multup(&p).equals( Tuple::point(Number::from(SQRT_2 / 2.0), 
                                                              Number::from(0),
                                                              Number::from(SQRT_2 / 2.0))));
        assert!( full_quarter.multup(&p).equals( Tuple::point(Number::from(1), 
                                                              Number::from(0), 
                                                              Number::from(0))));
    }

    #[test]
    fn rotate_point_around_z_axis(){
        let p = Tuple::point(Number::from(0), 
                             Number::from(1), 
                             Number::from(0));
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);

        assert!( half_quarter.multup(&p).equals( Tuple::point(Number::from(-SQRT_2 / 2.0), 
                                                              Number::from( SQRT_2 / 2.0),
                                                              Number::from(0))));
        assert!( full_quarter.multup(&p).equals( Tuple::point(Number::from(-1), 
                                                              Number::from(0), 
                                                              Number::from(0))));
    }

    #[test]
    fn shearing_moves_x_proportionate_to_y(){
        let t = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(Number::from(2), 
                             Number::from(3), 
                             Number::from(4));

        assert!( t.multup(&p).equals( Tuple::point(Number::from(5.0), 
                                                   Number::from(3.0),
                                                   Number::from(4.0))));
    }

    #[test]
    fn shearing_moves_x_proportionate_to_z(){
        let t = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(Number::from(2), 
                             Number::from(3), 
                             Number::from(4));

        assert!( t.multup(&p).equals( Tuple::point(Number::from(6.0), 
                                                   Number::from(3.0),
                                                   Number::from(4.0))));
    }

    #[test]
    fn shearing_moves_y_proportionate_to_x(){
        let t = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(Number::from(2), 
                             Number::from(3), 
                             Number::from(4));

        assert!( t.multup(&p).equals( Tuple::point(Number::from(2.0), 
                                                   Number::from(5.0),
                                                   Number::from(4.0))));
    }
    
    #[test]
    fn shearing_moves_y_proportionate_to_z(){
        let t = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(Number::from(2), 
                             Number::from(3), 
                             Number::from(4));

        assert!( t.multup(&p).equals( Tuple::point(Number::from(2.0), 
                                                   Number::from(7.0),
                                                   Number::from(4.0))));
    }

    #[test]
    fn shearing_moves_z_proportionate_to_x(){
        let t = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(Number::from(2), 
                             Number::from(3), 
                             Number::from(4));

        assert!( t.multup(&p).equals( Tuple::point(Number::from(2.0), 
                                                   Number::from(3.0),
                                                   Number::from(6.0))));
    }

    #[test]
    fn shearing_moves_z_proportionate_to_y(){
        let t = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(Number::from(2), 
                             Number::from(3), 
                             Number::from(4));

        assert!( t.multup(&p).equals( Tuple::point(Number::from(2.0), 
                                                   Number::from(3.0),
                                                   Number::from(7.0))));
    }

    #[test]
    fn individual_transforms_apply_in_sequence(){
        let p = Tuple::point(Number::from(1), 
                             Number::from(0), 
                             Number::from(1));
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let p2 = a.multup(&p);
        assert!( p2.equals( Tuple::point(Number::from( 1), 
                                         Number::from(-1), 
                                         Number::from( 0))));

        let p3 = b.multup(&p2);
        assert!( p3.equals( Tuple::point(Number::from( 5), 
                                         Number::from(-5), 
                                         Number::from( 0))));

        let p4 = c.multup(&p3);
        assert!( p4.equals( Tuple::point(Number::from(15), 
                                         Number::from( 0), 
                                         Number::from( 7))));
    }
}
