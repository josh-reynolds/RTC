use crate::color::Color;
use crate::tuple::Tuple;
use crate::shapes::Shape;
use crate::matrix::{Matrix, identity};
use core::fmt::Debug;

//pub const WHITE : Color = color(1.0, 1.0, 1.0);
//pub const BLACK : Color = color(0.0, 0.0, 0.0);

//static WHITE : Color = color(1.0, 1.0, 1.0);
//static BLACK : Color = color(0.0, 0.0, 0.0);

// neither of the previous work as-is, need to 
// research alternatives (like once_cell)

#[derive(Debug,Clone,PartialEq)]
pub struct Base {
    a: Color,
    b: Color,
    transform: Matrix,
    index: usize,
}

impl Pattern for Base {
    fn stripe_at(&self, _p: Tuple) -> Color {
        self.a
    }

    fn set_pattern_transform(&mut self, t: Matrix){
        self.transform = t
    }

    fn get_pattern_transform(&self) -> Matrix {
        self.transform.clone()
    }

    fn get_color_a(&self) -> Color {
        self.a
    }

    fn get_color_b(&self) -> Color {
        self.b
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize){
        self.index = index;
    }
}

pub fn pattern(a: Color, b: Color) -> Base {
    Base { a, b, transform: identity(), index: 0 }
}

pub trait Pattern {
    fn stripe_at(&self, p: Tuple) -> Color;

    fn stripe_at_object(&self, o: &Box<dyn Shape>, p: Tuple) -> Color {
        let object_point = o.get_transform().inverse().multup( &p );
        let pattern_point = self.get_pattern_transform().inverse().multup( &object_point );
        self.stripe_at( pattern_point )
    }
    
    fn set_pattern_transform(&mut self, t: Matrix);
    fn get_pattern_transform(&self) -> Matrix;

    fn get_color_a(&self) -> Color;
    fn get_color_b(&self) -> Color;
    fn get_index(&self) -> usize;
    fn set_index(&mut self, index: usize);
}

impl Debug for dyn Pattern {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Pattern {}", self.get_index())
    }
}

#[cfg(test)]
mod tests {
    use crate::patterns::{Pattern, pattern};
    use crate::color::color;
    use crate::transform::translation;
    use crate::matrix::identity;

    #[test]
    fn default_pattern_transform(){
        let white = color(1.0, 1.0, 1.0);
        let black = color(0.0, 0.0, 0.0);
        let p = pattern(white, black);

        assert_eq!( p.get_pattern_transform(), identity() );
    }

    #[test]
    fn pattern_transform_can_be_set(){
        let white = color(1.0, 1.0, 1.0);
        let black = color(0.0, 0.0, 0.0);
        let mut p = pattern(white, black);

        p.set_pattern_transform( translation(1.0, 2.0, 3.0) );

        assert_eq!( p.get_pattern_transform(), translation(1.0, 2.0, 3.0) );
    }
}
