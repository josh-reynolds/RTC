use crate::equals;
use std::ops::{Add, Sub, Mul};

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Add for Color {
    type Output = Self;

    fn add(self, c: Self) -> Self {
        Self { r: self.r + c.r,
               g: self.g + c.g,
               b: self.b + c.b, }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, c: Self) -> Self {
        Self { r: self.r - c.r,
               g: self.g - c.g,
               b: self.b - c.b, }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, c: Self) -> Self {
        Self { r: self.r * c.r,
               g: self.g * c.g,
               b: self.b * c.b }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self { r: self.r * rhs,
               g: self.g * rhs,
               b: self.b * rhs }
    }
}

impl Color {
    pub fn equals(&self, c: Color) -> bool {
        equals( self.r, c.r ) && 
        equals( self.g, c.g ) &&
        equals( self.b, c.b ) 
    }
}

pub fn color( r: f64, g: f64, b: f64 ) -> Color {
    Color { r, g, b }
}

#[cfg(test)]
mod tests {
    use crate::equals;
    use crate::color::color;

    #[test]
    fn colors_are_r_g_b_tuples(){
        let c = color( -0.5, 0.4, 1.7 );
        assert!( equals(c.r, -0.5) &&
                 equals(c.g, 0.4) &&
                 equals(c.b, 1.7));
    }

    #[test]
    fn adding_colors(){
        let c1 = color( 0.9, 0.6, 0.75 );
        let c2 = color( 0.7, 0.1, 0.25 );
        assert!( (c1 + c2).equals(color( 1.6, 0.7, 1.0 )));
    }

    #[test]
    fn subtracting_colors(){
        let c1 = color( 0.9, 0.6, 0.75 );
        let c2 = color( 0.7, 0.1, 0.25 );
        assert!( (c1 - c2).equals(color( 0.2, 0.5, 0.5 )));
    }

    // in the text, this is multiplied by an integer, but I think float will be needed
    #[test]
    fn multipy_color_by_scalar(){
        let c = color( 0.2, 0.3, 0.4 );
        assert!( (c * 2.0).equals( color( 0.4, 0.6, 0.8 ) ));
    }

    #[test]
    fn multiply_color_by_color(){
        let c1 = color( 1.0, 0.2, 0.4 );
        let c2 = color( 0.9, 1.0, 0.1 );
        assert!( (c1 * c2).equals( color( 0.9, 0.2, 0.04 ) ));
    }
}
