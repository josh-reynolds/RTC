use crate::matrix::{Matrix, identity};
use crate::rays::{Ray, ray};
use crate::tuple::{point, origin};
use crate::world::World;
use crate::canvas::{Canvas, canvas};

#[derive(Debug)]
pub struct Camera {
    pub hsize: i32,
    pub vsize: i32,
    pub field_of_view: f64,
    pub transform: Matrix,
    pub pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl Camera {
    pub fn ray_for_pixel(&self, px: i32, py: i32) -> Ray {
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = self.transform.inverse().multup( &point(world_x, world_y, -1.0) );
        let origin = self.transform.inverse().multup( &origin() );
        let direction = (pixel - origin).normal();

        ray( origin, direction )
    }

    pub fn render(&self, w: World) -> Canvas {
        let mut image = canvas(self.hsize.try_into().unwrap(), self.vsize.try_into().unwrap());

        for y in 0..(self.vsize - 1){
            for x in 0..(self.hsize - 1){
                let ray = self.ray_for_pixel(x, y);
                let color = w.color_at(ray);
                image.write_pixel(x.try_into().unwrap(), y.try_into().unwrap(), color);
            }
        }

        image
    }
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
            half_width,
            half_height,
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{PI, SQRT_2};
    use crate::camera::camera;
    use crate::matrix::identity;
    use crate::equals::equals;
    use crate::tuple::{point, vector};
    use crate::transform::{rotation_y, translation, view_transform};
    use crate::world::default_world;
    use crate::color::color;

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

    #[test]
    fn ray_through_center_of_canvas(){
        let c = camera(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        assert!( r.origin.equals( point(0.0, 0.0, 0.0) ));
        assert!( r.direction.equals( vector(0.0, 0.0, -1.0) ));
    }

    #[test]
    fn ray_through_corner_of_canvas(){
        let c = camera(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);
        assert!( r.origin.equals( point(0.0, 0.0, 0.0) ));
        assert!( r.direction.equals( vector(0.66519, 0.33259, -0.66851) ));
    }

    #[test]
    fn ray_when_camera_is_transformed(){
        let mut c = camera(201, 101, PI / 2.0);
        c.transform = rotation_y(PI / 4.0).mult( &translation(0.0, -2.0, 5.0) );
        let r = c.ray_for_pixel(100, 50);
        assert!( r.origin.equals( point(0.0, 2.0, -5.0) ));
        assert!( r.direction.equals( vector(SQRT_2 / 2.0, 0.0, -SQRT_2 / 2.0) ));
    }

    #[test]
    fn rendering_a_world_with_a_camera(){
        let w = default_world();
        
        let mut c = camera(11, 11, PI / 2.0);
        let from = point(0.0, 0.0, -5.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        c.transform = view_transform(from, to, up);

        let image = c.render(w);
        assert!( image.pixel_at(5, 5).equals( color(0.38066, 0.47583, 0.2855) ));
    }
}
