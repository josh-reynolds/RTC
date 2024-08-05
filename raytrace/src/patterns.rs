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
pub struct Stripes {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix,
}

impl Stripes {
    pub fn stripe_at(&self, p: Tuple) -> Color {
        if p.x.floor() as i64 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }

    pub fn stripe_at_object(&self, o: &Box<dyn Shape>, p: Tuple) -> Color {
        let object_point = o.get_transform().inverse().multup( &p );
        let pattern_point = self.get_pattern_transform().inverse().multup( &object_point );
        self.stripe_at( pattern_point )
    }
    
    pub fn set_pattern_transform(&mut self, t: Matrix){
        self.transform = t
    }

    pub fn get_pattern_transform(&self) -> Matrix {
        self.transform.clone()
    }
}

pub fn stripe_pattern(a: Color, b: Color) -> Stripes {
    Stripes { a, b, transform: identity() }
}

#[cfg(test)]
mod tests {
    use crate::patterns::stripe_pattern;
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
        assert_eq!(p.a, white);
        assert_eq!(p.b, black);
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
