use crate::color::{Color, color};
use crate::equals::equals;
use crate::lights::Light;
use crate::tuple::Tuple;
use crate::patterns::{Pattern, Stripes};
use crate::shapes::Shape;

#[derive(Debug,Clone,PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,      // typical range 0-1
    pub diffuse: f64,      // typical range 0-1
    pub specular: f64,     // typical range 0-1
    pub shininess: f64,    // typical range 10-200
    pub pattern: Option<Stripes>,
}

impl Material {
    pub fn equals(&self, m: Material) -> bool {
        self.color.equals( m.color ) &&
        equals(self.ambient, m.ambient) &&
        equals(self.diffuse, m.diffuse) &&
        equals(self.specular, m.specular) &&
        equals(self.shininess, m.shininess) &&
        self.pattern == m.pattern
    }
}

pub fn material() -> Material {
    Material {
        color: color(1.0, 1.0, 1.0),
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0,
        pattern: None,
    }
}

pub fn lighting(m: Material, 
                o: &Box<dyn Shape>,
                l: &Light, 
                p: Tuple, 
                eye: Tuple, 
                normal: Tuple, 
                in_shadow: bool
  ) -> Color {
    let true_color = match m.pattern {
        Some(pattern) => pattern.stripe_at_object(o, p),
        None          => m.color,
    };
    let effective_color = true_color * l.intensity;
    let lightv = (l.position - p).normal();
    let ambient = effective_color * m.ambient;

    let light_dot_normal = lightv.dot( &normal );
    let mut diffuse = color(0.0, 0.0, 0.0);
    let mut specular = color(0.0, 0.0, 0.0);

    if light_dot_normal >= 0.0 {
        diffuse = effective_color * m.diffuse * light_dot_normal;

        let reflectv = -lightv.reflect(&normal);
        let reflect_dot_eye = reflectv.dot( &eye );

        if reflect_dot_eye > 0.0 {
            let factor = reflect_dot_eye.powf( m.shininess );
            specular = l.intensity * m.specular * factor;
        }
    }
    
    if in_shadow {
        ambient
    } else {
        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use crate::equals::equals;
    use crate::color::color;
    use crate::materials::{material, lighting};
    use crate::tuple::{origin, point, vector};
    use crate::lights::point_light;
    use std::f64::consts::SQRT_2;
    use crate::patterns::stripe_pattern;
    use crate::spheres::sphere;
    use crate::shapes::Shape;

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

        let result = lighting(m, 
                              &(Box::new(sphere()) as Box<dyn Shape>),
                              &light, 
                              p, 
                              eyev, 
                              normalv, 
                              false);
        assert!( result.equals(color(1.9, 1.9, 1.9)) );
    }

    #[test]
    fn lighting_eye_between_light_and_surface_eye_offset_45(){
        let m = material();
        let p = origin();

        let eyev = vector(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));

        let result = lighting(m, 
                              &(Box::new(sphere()) as Box<dyn Shape>),
                              &light, 
                              p, 
                              eyev, 
                              normalv, 
                              false);
        assert!( result.equals(color(1.0, 1.0, 1.0)) );
    }

    #[test]
    fn lighting_eye_opposite_surface_light_offset_45(){
        let m = material();
        let p = origin();

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 10.0, -10.0), color(1.0, 1.0, 1.0));

        let result = lighting(m, 
                              &(Box::new(sphere()) as Box<dyn Shape>),
                              &light, 
                              p, eyev, 
                              normalv, 
                              false);
        assert!( result.equals(color(0.7364, 0.7364, 0.7364)) );
    }

    #[test]
    fn lighting_eye_in_path_of_reflection(){
        let m = material();
        let p = origin();

        let eyev = vector(0.0, -SQRT_2 / 2.0, -SQRT_2 / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 10.0, -10.0), color(1.0, 1.0, 1.0));

        let result = lighting(m, 
                              &(Box::new(sphere()) as Box<dyn Shape>),
                              &light, 
                              p, 
                              eyev, 
                              normalv, 
                              false);
        assert!( result.equals(color(1.6364, 1.6364, 1.6364)) );
    }

    #[test]
    fn lighting_light_behind_surface(){
        let m = material();
        let p = origin();

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, 10.0), color(1.0, 1.0, 1.0));

        let result = lighting(m, 
                              &(Box::new(sphere()) as Box<dyn Shape>),
                              &light, 
                              p, 
                              eyev, 
                              normalv, 
                              false);
        assert!( result.equals(color(0.1, 0.1, 0.1)) );
    }

    #[test]
    fn lighting_with_surface_in_shadow(){
        let m = material();
        let p = origin();

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));
        let in_shadow = true;

        let result = lighting(m, 
                              &(Box::new(sphere()) as Box<dyn Shape>),
                              &light, 
                              p, 
                              eyev, 
                              normalv, 
                              in_shadow);
        assert!( result.equals(color(0.1, 0.1, 0.1)) );
    }

    #[test]
    fn lighting_with_pattern_applied(){
        let mut m = material();
        m.pattern = Some(stripe_pattern(color(1.0, 1.0, 1.0), color(0.0, 0.0, 0.0)));
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));
        
        let c1 = lighting(m.clone(), 
                          &(Box::new(sphere()) as Box<dyn Shape>),
                          &light, 
                          point(0.9, 0.0, 0.0), 
                          eyev, 
                          normalv, 
                          false);
        let c2 = lighting(m.clone(), 
                          &(Box::new(sphere()) as Box<dyn Shape>),
                          &light, 
                          point(1.1, 0.0, 0.0), 
                          eyev, 
                          normalv, 
                          false);

        assert_eq!(c1, color(1.0, 1.0, 1.0));
        assert_eq!(c2, color(0.0, 0.0, 0.0));
    }
}
