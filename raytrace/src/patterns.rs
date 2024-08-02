use crate::color::Color;

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

pub fn stripe_pattern(a: Color, b: Color) -> Pattern {
    Pattern { a, b }
}

#[cfg(test)]
mod tests {
    use crate::patterns::stripe_pattern;
    use crate::color::color;

    #[test]
    fn creating_a_stripe_pattern(){
        let white = color(1.0, 1.0, 1.0);
        let black = color(0.0, 0.0, 0.0);
        let p = stripe_pattern(white, black);
        assert_eq!(p.a, white);
        assert_eq!(p.b, black);
    }
}
