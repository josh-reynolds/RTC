use crate::matrix::{Matrix, identity};

#[derive(Debug)]
pub struct Camera {
    pub hsize: i32,
    pub vsize: i32,
    pub field_of_view: f64,
    pub transform: Matrix,
}

pub fn camera(hsize: i32, vsize: i32, field_of_view: f64) -> Camera {
    Camera{ hsize, 
            vsize, 
            field_of_view,
            transform: identity(), }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use crate::camera::camera;
    use crate::matrix::identity;
    use crate::equals::equals;

    #[test]
    fn constructing_a_camera(){
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;

        let c = camera(hsize, vsize, field_of_view);
        assert!(c.hsize == hsize);
        assert!(c.vsize == vsize);
        assert!(equals(c.field_of_view, field_of_view));
        assert!(c.transform == identity());

    }
}
