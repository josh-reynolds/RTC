use crate::matrix::{Matrix, identity};

#[derive(Debug)]
pub struct Camera {
    pub hsize: i32,
    pub vsize: i32,
    pub field_of_view: f64,
    pub transform: Matrix,
    pub pixel_size: f64,
}

pub fn camera(hsize: i32, vsize: i32, field_of_view: f64) -> Camera {
    let half_view = (field_of_view / 2.0).tan();
    let aspect = hsize as f64 / vsize as f64;
    let mut half_height = half_view;
    let mut half_width = half_view * aspect;

    if aspect >= 1.0 {
        half_width = half_view;
        half_height = half_view / aspect;
    }

    Camera{ hsize, 
            vsize, 
            field_of_view,
            transform: identity(),
            pixel_size: (half_width * 2.0) / hsize as f64,
    }
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

    #[test]
    fn pixel_size_for_horizontal_canvas(){
        let c = camera(200, 125, PI / 2.0);
        assert!( equals( c.pixel_size, 0.01 ));
    }

    #[test]
    fn pixel_size_for_vertical_canvas(){
        let c = camera(125, 200, PI / 2.0);
        assert!( equals( c.pixel_size, 0.01 ));
    }
}
