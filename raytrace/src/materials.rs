use crate::color::{Color, color};
use crate::equals::equals;

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

#[cfg(test)]
mod tests {
    use crate::equals::equals;
    use crate::color::color;
    use crate::materials::material;

    #[test]
    fn default_material(){
        let m = material();

        assert!( m.color.equals( color(1.0, 1.0, 1.0)));
        assert!( equals( m.ambient, 0.1 ));
        assert!( equals( m.diffuse, 0.9 ));
        assert!( equals( m.specular, 0.9 ));
        assert!( equals( m.shininess, 200.0 ));
    }
}
