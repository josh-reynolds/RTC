use crate::color::Color;
use crate::matrix::Matrix;
use crate::tuple::Tuple;
use crate::patterns::{Base, Pattern, pattern};

#[derive(Debug,Clone,PartialEq)]
pub struct Gradient {
    supe: Base,
}

impl Pattern for Gradient {
    fn pattern_at(&self, p: Tuple) -> Color {
        let distance = self.supe.get_color_b() - self.supe.get_color_a();
        let fraction = p.x - p.x.floor();

        self.supe.get_color_a() + distance * fraction
    }

    fn set_pattern_transform(&mut self, t: Matrix){
        self.supe.set_pattern_transform( t )
    }

    fn get_pattern_transform(&self) -> Matrix {
        self.supe.get_pattern_transform()
    }

    fn get_color_a(&self) -> Color {
        self.supe.get_color_a()
    }

    fn get_color_b(&self) -> Color {
        self.supe.get_color_b()
    }

    fn get_index(&self) -> usize {
        self.supe.get_index()
    }

    fn set_index(&mut self, index: usize){
        self.supe.set_index(index);
    }
}

pub fn gradient_pattern(a: Color, b: Color) -> Gradient {
    Gradient { supe: pattern(a, b) }
}

#[cfg(test)]
mod tests {
    use crate::color::color;
    use crate::gradients::gradient_pattern;
    use crate::tuple::point;
    use crate::patterns::Pattern;
    
    #[test]
    fn creating_a_gradient_pattern(){
        let white = color(1.0, 1.0, 1.0);
        let black = color(0.0, 0.0, 0.0);
        let p = gradient_pattern(white, black);
        assert_eq!(p.get_color_a(), white);
        assert_eq!(p.get_color_b(), black);
    }

    #[test]
    fn gradient_linearly_interpolates_between_colors(){
        let white = color(1.0, 1.0, 1.0);
        let black = color(0.0, 0.0, 0.0);
        let p = gradient_pattern(white, black);

        assert_eq!(p.pattern_at( point( 0.0, 0.0, 0.0) ), white);
        assert_eq!(p.pattern_at( point(0.25, 0.0, 0.0) ), color(0.75, 0.75, 0.75));
        assert_eq!(p.pattern_at( point( 0.5, 0.0, 0.0) ), color(0.5, 0.5, 0.5));
        assert_eq!(p.pattern_at( point(0.75, 0.0, 0.0) ), color(0.25, 0.25, 0.25));
    }
} 
