use crate::tuple::Tuple;
use crate::color::Color;

#[derive(Debug)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Color,
}

impl Light {
    pub fn equals(&self, l: Light) -> bool {
        self.position.equals( l.position ) &&
        self.intensity.equals( l.intensity )
    }
}

pub fn point_light(position: Tuple, intensity: Color) -> Light {
    Light { position: position, intensity: intensity }
}


#[cfg(test)]
mod tests {
    use crate::color::color;
    use crate::tuple::point;
    use crate::lights::point_light;

    #[test]
    fn point_light_has_position_and_intensity(){
        let position = point(0.0, 0.0, 0.0);
        let intensity = color(1.0, 1.0, 1.0);
        let light = point_light(position, intensity);
        assert!( light.position.equals( position ));
        assert!( light.intensity.equals( intensity ));
    }
}
