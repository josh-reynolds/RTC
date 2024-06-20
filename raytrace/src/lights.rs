use crate::tuple::Tuple;
use crate::color::Color;

#[derive(Debug)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Color,
}

pub fn point_light(position: Tuple, intensity: Color) -> Light {
    Light { position: position, intensity: intensity }
}


#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::tuple::point;
    use crate::lights::point_light;

    #[test]
    fn point_light_has_position_and_intensity(){
        let intensity = Color { r: 1.0, g: 1.0, b: 1.0 };
        let position = point(0.0, 0.0, 0.0);
        let light = point_light(position, intensity);
        assert!( light.position.equals( position ));
        assert!( light.intensity.equals( intensity ));

    }
}
