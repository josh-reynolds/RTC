use crate::matrix::{Matrix, identity};

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    let mut t = identity();
    t.set(0,3,x);
    t.set(1,3,y);
    t.set(2,3,z);
    t
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    let mut t = identity();
    t.set(0,0,x);
    t.set(1,1,y);
    t.set(2,2,z);
    t
}

pub fn rotation_x(r: f64) -> Matrix {
    let mut t = identity();
    t.set(1,1,  r.cos());
    t.set(1,2, -r.sin());
    t.set(2,1,  r.sin());
    t.set(2,2,  r.cos());
    t
}

pub fn rotation_y(r: f64) -> Matrix {
    let mut t = identity();
    t.set(0,0,  r.cos());
    t.set(0,2,  r.sin());
    t.set(2,0, -r.sin());
    t.set(2,2,  r.cos());
    t
}

pub fn rotation_z(r: f64) -> Matrix {
    let mut t = identity();
    t.set(0,0,  r.cos());
    t.set(0,1, -r.sin());
    t.set(1,0,  r.sin());
    t.set(1,1,  r.cos());
    t
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    let mut t = identity();
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
    use crate::tuple::{point, vector};
    use crate::transform::*;
    use std::f64::consts::{PI, SQRT_2};

    #[test]
    fn multiply_by_translation_matrix(){
        let t = translation(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);

        assert!( t.multup(&p).equals( point(2.0, 1.0, 7.0)));
    }

    #[test]
    fn multiply_by_inverse_translation_matrix(){
        let t = translation(5.0, -3.0, 2.0);
        let inv = t.inverse();
        let p = point(-3.0, 4.0, 5.0);

        assert!( inv.multup(&p).equals( point(-8.0, 7.0, 3.0)));
    }

    #[test]
    fn translation_does_not_affect_vectors(){
        let t = translation(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);

        assert!( t.multup(&v).equals( v ));
    }

    #[test]
    fn multiply_point_by_scaling_matrix(){
        let t = scaling(2.0, 3.0, 4.0);
        let p = point(-4.0, 6.0, 8.0);

        assert!( t.multup(&p).equals( point(-8.0, 18.0, 32.0)));
    }

    #[test]
    fn multiply_vector_by_scaling_matrix(){
        let t = scaling(2.0, 3.0, 4.0);
        let v = vector(-4.0, 6.0, 8.0);

        assert!( t.multup(&v).equals( vector(-8.0, 18.0, 32.0)));
    }

    #[test]
    fn multiply_vector_by_inverse_scaling_matrix(){
        let t = scaling(2.0, 3.0, 4.0);
        let inv = t.inverse();
        let v = vector(-4.0, 6.0, 8.0);

        assert!( inv.multup(&v).equals( vector(-2.0, 2.0, 2.0)));
    }

    #[test]
    fn reflection_is_scaling_by_negative_value(){
        let t = scaling(-1.0, 1.0, 1.0);
        let p = point(2.0, 3.0, 4.0);

        assert!( t.multup(&p).equals( point(-2.0, 3.0, 4.0)));
    }

    #[test]
    fn rotate_point_around_x_axis(){
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        assert!( half_quarter.multup(&p).equals( point(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0)));
        assert!( full_quarter.multup(&p).equals( point(0.0, 0.0, 1.0)));
    }

    #[test]
    fn inverse_of_rotation_rotates_opposite_direction(){
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let inv = half_quarter.inverse();

        assert!( inv.multup(&p).equals( point(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0)));
    }

    #[test]
    fn rotate_point_around_y_axis(){
        let p = point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);

        assert!( half_quarter.multup(&p).equals( point(SQRT_2 / 2.0, 0.0, SQRT_2 / 2.0)));
        assert!( full_quarter.multup(&p).equals( point(1.0, 0.0, 0.0)));
    }

    #[test]
    fn rotate_point_around_z_axis(){
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);

        assert!( half_quarter.multup(&p).equals( point(-SQRT_2 / 2.0, SQRT_2 / 2.0, 0.0)));
        assert!( full_quarter.multup(&p).equals( point(-1.0, 0.0, 0.0)));
    }

    #[test]
    fn shearing_moves_x_proportionate_to_y(){
        let t = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert!( t.multup(&p).equals( point(5.0, 3.0, 4.0)));
    }

    #[test]
    fn shearing_moves_x_proportionate_to_z(){
        let t = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert!( t.multup(&p).equals( point(6.0, 3.0, 4.0)));
    }

    #[test]
    fn shearing_moves_y_proportionate_to_x(){
        let t = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert!( t.multup(&p).equals( point(2.0, 5.0, 4.0)));
    }
    
    #[test]
    fn shearing_moves_y_proportionate_to_z(){
        let t = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert!( t.multup(&p).equals( point(2.0, 7.0, 4.0)));
    }

    #[test]
    fn shearing_moves_z_proportionate_to_x(){
        let t = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert!( t.multup(&p).equals( point(2.0, 3.0, 6.0)));
    }

    #[test]
    fn shearing_moves_z_proportionate_to_y(){
        let t = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = point(2.0, 3.0, 4.0);

        assert!( t.multup(&p).equals( point(2.0, 3.0, 7.0)));
    }

    #[test]
    fn individual_transforms_apply_in_sequence(){
        let p = point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let p2 = a.multup(&p);
        assert!( p2.equals( point( 1.0, -1.0, 0.0)));

        let p3 = b.multup(&p2);
        assert!( p3.equals( point( 5.0, -5.0, 0.0)));

        let p4 = c.multup(&p3);
        assert!( p4.equals( point(15.0, 0.0, 7.0)));
    }

    #[test]
    fn chained_transforms_apply_in_reverse_order(){
        let p = point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let t = c.mult( &b.mult( &a ));

        assert!( t.multup(&p).equals( point(15.0, 0.0, 7.0)));
    }
}
