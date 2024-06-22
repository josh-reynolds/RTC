use crate::color::{Color, color};
use crate::equals::equals;
use crate::lights::Light;
use crate::tuple::Tuple;

#[derive(Debug,Clone,Copy)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,      // typical range 0-1
    pub diffuse: f64,      // typical range 0-1
    pub specular: f64,     // typical range 0-1
    pub shininess: f64,    // typical range 10-200
}

impl Material {
    pub fn equals(&self, m: Material) -> bool {
        self.color.equals( m.color ) &&
        equals(self.ambient, m.ambient) &&
        equals(self.diffuse, m.diffuse) &&
        equals(self.specular, m.specular) &&
        equals(self.shininess, m.shininess) 
    }
}

pub fn material() -> Material {
    Material {
        color: color(1.0, 1.0, 1.0),
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0,
    }
}

pub fn lighting(m: Material, l: &Light, p: Tuple, eye: Tuple, normal: Tuple) -> Color {
    let effective_color = m.color * l.intensity;
    let lightv = (l.position - p).normal();
    let ambient = effective_color.mult( m.ambient );

    let light_dot_normal = lightv.dot( &normal );
    let mut diffuse = color(0.0, 0.0, 0.0);
    let mut specular = color(0.0, 0.0, 0.0);

    if light_dot_normal >= 0.0 {
        diffuse = effective_color.mult( m.diffuse ).mult( light_dot_normal );

        let reflectv = -lightv.reflect(&normal);
        let reflect_dot_eye = reflectv.dot( &eye );

        if reflect_dot_eye > 0.0 {
            let factor = reflect_dot_eye.powf( m.shininess );
            specular = l.intensity.mult( m.specular).mult( factor );
        }
    }
    
    ambient + diffuse + specular
}

#[cfg(test)]
mod tests {
    use crate::equals::equals;
    use crate::color::color;
    use crate::materials::{material, lighting};
    use crate::tuple::{origin, point, vector};
    use crate::lights::point_light;
    use std::f64::consts::SQRT_2;

    #[test]
    fn default_material(){
        let m = material();

        assert!( m.color.equals( color(1.0, 1.0, 1.0)));
        assert!( equals( m.ambient, 0.1 ));
        assert!( equals( m.diffuse, 0.9 ));
        assert!( equals( m.specular, 0.9 ));
        assert!( equals( m.shininess, 200.0 ));
    }

    #[test]
    fn lighting_eye_between_light_and_surface(){
        let m = material();
        let p = origin();

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));

        let result = lighting(m, &light, p, eyev, normalv);
        assert!( result.equals(color(1.9, 1.9, 1.9)) );
    }

    #[test]
    fn lighting_eye_between_light_and_surface_eye_offset_45(){
        let m = material();
        let p = origin();

        let eyev = vector(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));

        let result = lighting(m, &light, p, eyev, normalv);
        assert!( result.equals(color(1.0, 1.0, 1.0)) );
    }

    #[test]
    fn lighting_eye_opposite_surface_light_offset_45(){
        let m = material();
        let p = origin();

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 10.0, -10.0), color(1.0, 1.0, 1.0));

        let result = lighting(m, &light, p, eyev, normalv);
        assert!( result.equals(color(0.7364, 0.7364, 0.7364)) );
    }

    #[test]
    fn lighting_eye_in_path_of_reflection(){
        let m = material();
        let p = origin();

        let eyev = vector(0.0, -SQRT_2 / 2.0, -SQRT_2 / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 10.0, -10.0), color(1.0, 1.0, 1.0));

        let result = lighting(m, &light, p, eyev, normalv);
        assert!( result.equals(color(1.6364, 1.6364, 1.6364)) );
    }

    #[test]
    fn lighting_light_behind_surface(){
        let m = material();
        let p = origin();

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, 10.0), color(1.0, 1.0, 1.0));

        let result = lighting(m, &light, p, eyev, normalv);
        assert!( result.equals(color(0.1, 0.1, 0.1)) );
    }
}
