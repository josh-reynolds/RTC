use crate::color::Color;
use crate::matrix::Matrix;
use crate::tuple::Tuple;
use crate::patterns::{Base, Pattern, pattern};

#[derive(Debug,Clone,PartialEq)]
pub struct Checker {
    supe: Base,
}

impl Pattern for Checker {
    fn pattern_at(&self, p: Tuple) -> Color {
        let sum = p.x.floor() + p.y.floor() + p.z.floor();
        if sum % 2.0 == 0.0 {
            self.get_color_a()
        } else {
            self.get_color_b()
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

pub fn checker_pattern(a: Color, b: Color) -> Checker {
    Checker { supe: pattern(a, b) }
}

#[cfg(test)]
mod tests {
    use crate::color::color;
    use crate::checkers::checker_pattern;
    use crate::tuple::point;
    use crate::patterns::Pattern;
    
    #[test]
    fn creating_a_checker_pattern(){
        let white = color(1.0, 1.0, 1.0);
        let black = color(0.0, 0.0, 0.0);
        let p = checker_pattern(white, black);
        assert_eq!(p.get_color_a(), white);
        assert_eq!(p.get_color_b(), black);
    }

    #[test]
    fn checker_repeats_in_x(){
        let white = color(1.0, 1.0, 1.0);
        let black = color(0.0, 0.0, 0.0);
        let p = checker_pattern(white, black);

        assert_eq!(p.pattern_at(point(0.0,  0.0, 0.0)), white);
        assert_eq!(p.pattern_at(point(0.99, 0.0, 0.0)), white);
        assert_eq!(p.pattern_at(point(1.01, 0.0, 0.0)), black);
    }

    #[test]
    fn checker_repeats_in_y(){
        let white = color(1.0, 1.0, 1.0);
        let black = color(0.0, 0.0, 0.0);
        let p = checker_pattern(white, black);

        assert_eq!(p.pattern_at(point(0.0, 0.0,  0.0)), white);
        assert_eq!(p.pattern_at(point(0.0, 0.99, 0.0)), white);
        assert_eq!(p.pattern_at(point(0.0, 1.01, 0.0)), black);
    }

    #[test]
    fn checker_repeats_in_z(){
        let white = color(1.0, 1.0, 1.0);
        let black = color(0.0, 0.0, 0.0);
        let p = checker_pattern(white, black);

        assert_eq!(p.pattern_at(point(0.0, 0.0, 0.0)), white);
        assert_eq!(p.pattern_at(point(0.0, 0.0, 0.99)), white);
        assert_eq!(p.pattern_at(point(0.0, 0.0, 1.01)), black);
    }
} 
