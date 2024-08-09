use crate::color::Color;
use crate::tuple::Tuple;
use crate::shapes::Shape;
use crate::matrix::{Matrix, identity};

//pub const WHITE : Color = color(1.0, 1.0, 1.0);
//pub const BLACK : Color = color(0.0, 0.0, 0.0);

//static WHITE : Color = color(1.0, 1.0, 1.0);
//static BLACK : Color = color(0.0, 0.0, 0.0);

// neither of the previous work as-is, need to 
// research alternatives (like once_cell)

#[derive(Debug,Clone,PartialEq)]
struct Base {
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

fn pattern(a: Color, b: Color) -> Base {
    Base { a, b, transform: identity(), index: 0 }
}

#[derive(Debug,Clone,PartialEq)]
pub struct Stripes {
    supe: Base,
}

impl Pattern for Stripes {
    fn stripe_at(&self, p: Tuple) -> Color {
        if p.x.floor() as i64 % 2 == 0 {
            self.supe.a
        } else {
            self.supe.b
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

#[cfg(test)]
mod tests {
    use crate::patterns::{Pattern, stripe_pattern};
    use crate::color::color;
    use crate::tuple::point;
    use crate::spheres::sphere;
    use crate::shapes::Shape;
    use crate::transform::{scaling, translation};

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

        assert_eq!(p.stripe_at( point(0.0, 0.0, 0.0) ), white);
        assert_eq!(p.stripe_at( point(0.0, 1.0, 0.0) ), white);
        assert_eq!(p.stripe_at( point(0.0, 2.0, 0.0) ), white);
    }
    
    #[test]
    fn stripe_pattern_constant_in_z(){
        let white = color(1.0, 1.0, 1.0);
        let black = color(0.0, 0.0, 0.0);
        let p = stripe_pattern(white, black);

        assert_eq!(p.stripe_at( point(0.0, 0.0, 0.0) ), white);
        assert_eq!(p.stripe_at( point(0.0, 0.0, 1.0) ), white);
        assert_eq!(p.stripe_at( point(0.0, 0.0, 2.0) ), white);
    }

    #[test]
    fn stripe_pattern_alternates_in_x(){
        let white = color(1.0, 1.0, 1.0);
        let black = color(0.0, 0.0, 0.0);
        let p = stripe_pattern(white, black);

        assert_eq!(p.stripe_at( point( 0.0, 0.0, 0.0) ), white);
        assert_eq!(p.stripe_at( point( 0.9, 0.0, 0.0) ), white);
        assert_eq!(p.stripe_at( point( 1.0, 0.0, 0.0) ), black);
        assert_eq!(p.stripe_at( point(-0.1, 0.0, 0.0) ), black);
        assert_eq!(p.stripe_at( point(-1.0, 0.0, 0.0) ), black);
        assert_eq!(p.stripe_at( point(-1.1, 0.0, 0.0) ), white);
    }

    #[test]
    fn stripes_with_object_transformation(){
        let mut object = Box::new(sphere()) as Box<dyn Shape>;
        object.set_transform( scaling(2.0, 2.0, 2.0) );

        let white = color(1.0, 1.0, 1.0);
        let black = color(0.0, 0.0, 0.0);
        let p = stripe_pattern(white, black);

        let c = p.stripe_at_object( &object, point(1.5, 0.0, 0.0) );

        assert_eq!(c, white);
    }

    #[test]
    fn stripes_with_pattern_transformation(){
        let object = Box::new(sphere()) as Box<dyn Shape>;

        let white = color(1.0, 1.0, 1.0);
        let black = color(0.0, 0.0, 0.0);
        let mut p = stripe_pattern(white, black);
        p.set_pattern_transform( scaling(2.0, 2.0, 2.0) );

        let c = p.stripe_at_object( &object, point(1.5, 0.0, 0.0) );

        assert_eq!(c, white);
    }

    #[test]
    fn stripes_with_object_and_pattern_transformations(){
        let mut object = Box::new(sphere()) as Box<dyn Shape>;
        object.set_transform( scaling(2.0, 2.0, 2.0) );

        let white = color(1.0, 1.0, 1.0);
        let black = color(0.0, 0.0, 0.0);
        let mut p = stripe_pattern(white, black);
        p.set_pattern_transform( translation(0.5, 0.0, 0.0) );

        let c = p.stripe_at_object( &object, point(2.5, 0.0, 0.0) );

        assert_eq!(c, white);
    }
}
