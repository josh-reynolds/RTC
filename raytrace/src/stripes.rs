use crate::color::Color;
use crate::matrix::Matrix;
use crate::tuple::Tuple;
use crate::patterns::{Base, Pattern, pattern};

#[derive(Debug,Clone,PartialEq)]
pub struct Stripes {
    supe: Base,
}

impl Pattern for Stripes {
    fn pattern_at(&self, p: Tuple) -> Color {
        if p.x.floor() as i64 % 2 == 0 {
            self.supe.get_color_a()
        } else {
            self.supe.get_color_b()
        }
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

pub fn stripe_pattern(a: Color, b: Color) -> Stripes {
    Stripes { supe: pattern(a, b) }
}

#[cfg(test)]
mod tests {
    use crate::color::color;
    use crate::stripes::stripe_pattern;
    use crate::tuple::point;
    use crate::patterns::Pattern;
    
    #[test]
    fn creating_a_stripe_pattern(){
        let white = color(1.0, 1.0, 1.0);
        let black = color(0.0, 0.0, 0.0);
        let p = stripe_pattern(white, black);
        assert_eq!(p.get_color_a(), white);
        assert_eq!(p.get_color_b(), black);
    }

    #[test]
    fn stripe_pattern_constant_in_y(){
        let white = color(1.0, 1.0, 1.0);
        let black = color(0.0, 0.0, 0.0);
        let p = stripe_pattern(white, black);

        assert_eq!(p.pattern_at( point(0.0, 0.0, 0.0) ), white);
        assert_eq!(p.pattern_at( point(0.0, 1.0, 0.0) ), white);
        assert_eq!(p.pattern_at( point(0.0, 2.0, 0.0) ), white);
    }
    
    #[test]
    fn stripe_pattern_constant_in_z(){
        let white = color(1.0, 1.0, 1.0);
        let black = color(0.0, 0.0, 0.0);
        let p = stripe_pattern(white, black);

        assert_eq!(p.pattern_at( point(0.0, 0.0, 0.0) ), white);
        assert_eq!(p.pattern_at( point(0.0, 0.0, 1.0) ), white);
        assert_eq!(p.pattern_at( point(0.0, 0.0, 2.0) ), white);
    }

    #[test]
    fn stripe_pattern_alternates_in_x(){
        let white = color(1.0, 1.0, 1.0);
        let black = color(0.0, 0.0, 0.0);
        let p = stripe_pattern(white, black);

        assert_eq!(p.pattern_at( point( 0.0, 0.0, 0.0) ), white);
        assert_eq!(p.pattern_at( point( 0.9, 0.0, 0.0) ), white);
        assert_eq!(p.pattern_at( point( 1.0, 0.0, 0.0) ), black);
        assert_eq!(p.pattern_at( point(-0.1, 0.0, 0.0) ), black);
        assert_eq!(p.pattern_at( point(-1.0, 0.0, 0.0) ), black);
        assert_eq!(p.pattern_at( point(-1.1, 0.0, 0.0) ), white);
    }
} 
