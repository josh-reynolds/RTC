use crate::color::{Color, color};
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
    fn pattern_at(&self, p: Tuple) -> Color {
        color(p.x, p.y, p.z)
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
    fn pattern_at(&self, p: Tuple) -> Color;

    fn pattern_at_shape(&self, o: &Box<dyn Shape>, p: Tuple) -> Color {
        let object_point = o.get_transform().inverse().multup( &p );
        let pattern_point = self.get_pattern_transform().inverse().multup( &object_point );
        self.pattern_at( pattern_point )
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
    use crate::transform::{translation, scaling};
    use crate::matrix::identity;
    use crate::tuple::point;
    use crate::spheres::sphere;
    use crate::shapes::Shape;

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

    #[test]
    fn pattern_with_object_transform(){
        let mut s = Box::new(sphere()) as Box<dyn Shape>;
        s.set_transform( scaling(2.0, 2.0, 2.0) );
        
        let white = color(1.0, 1.0, 1.0);
        let black = color(0.0, 0.0, 0.0);
        let p = pattern(white, black);

        let c = p.pattern_at_shape( &s, point(2.0, 3.0, 4.0) );

        assert_eq!(c, color(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_pattern_transform(){
        let s = Box::new(sphere()) as Box<dyn Shape>;
        
        let white = color(1.0, 1.0, 1.0);
        let black = color(0.0, 0.0, 0.0);
        let mut p = pattern(white, black);
        p.set_pattern_transform( scaling(2.0, 2.0, 2.0) );

        let c = p.pattern_at_shape( &s, point(2.0, 3.0, 4.0) );

        assert_eq!(c, color(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_pattern_and_object_transforms(){
        let mut s = Box::new(sphere()) as Box<dyn Shape>;
        s.set_transform( scaling(2.0, 2.0, 2.0) );
        
        let white = color(1.0, 1.0, 1.0);
        let black = color(0.0, 0.0, 0.0);
        let mut p = pattern(white, black);
        p.set_pattern_transform( translation(0.5, 1.0, 1.5) );

        let c = p.pattern_at_shape( &s, point(2.5, 3.0, 3.5) );

        assert_eq!(c, color(0.75, 0.5, 0.25));
    }
}
