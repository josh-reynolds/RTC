use crate::color::Color;
use crate::tuple::Tuple;

//pub const WHITE : Color = color(1.0, 1.0, 1.0);
//pub const BLACK : Color = color(0.0, 0.0, 0.0);

//static WHITE : Color = color(1.0, 1.0, 1.0);
//static BLACK : Color = color(0.0, 0.0, 0.0);

// neither of the previous work as-is, need to 
// research alternatives (like once_cell)

#[derive(Debug)]
pub struct Pattern {
    pub a: Color,
    pub b: Color,
}

impl Pattern {
    pub fn stripe_at(&self, p: Tuple) -> Color {
        if p.x.floor() as i64 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

pub fn stripe_pattern(a: Color, b: Color) -> Pattern {
    Pattern { a, b }
}

#[cfg(test)]
mod tests {
    use crate::patterns::stripe_pattern;
    use crate::color::color;
    use crate::tuple::point;

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
}
