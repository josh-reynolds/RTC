use crate::equals;

#[derive(Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn plus(&self, c: &Color) -> Self {
        Self { r: self.r + c.r,
               g: self.g + c.g,
               b: self.b + c.b, }
    }

    pub fn sub(&self, c: &Color) -> Self {
        Self { r: self.r - c.r,
               g: self.g - c.g,
               b: self.b - c.b, }
    }
    
    pub fn equals(&self, c: Color) -> bool {
        equals( self.r, c.r ) && 
        equals( self.g, c.g ) &&
        equals( self.b, c.b ) 
    }
}

#[cfg(test)]
mod tests {
    use crate::equals;
    use crate::color::Color;

    #[test]
    fn colors_are_r_g_b_tuples(){
        let c = Color { r: -0.5, g: 0.4, b: 1.7 };
        assert!( equals(c.r, -0.5) &&
                 equals(c.g, 0.4) &&
                 equals(c.b, 1.7));
    }

    #[test]
    fn adding_colors(){
        let c1 = Color { r: 0.9, g: 0.6, b: 0.75 };
        let c2 = Color { r: 0.7, g: 0.1, b: 0.25 };
        assert!( c1.plus(&c2).equals(Color { r: 1.6, g: 0.7, b: 1.0 }));
    }
    
    #[test]
    fn subtracting_colors(){
        let c1 = Color { r: 0.9, g: 0.6, b: 0.75 };
        let c2 = Color { r: 0.7, g: 0.1, b: 0.25 };
        assert!( c1.sub(&c2).equals(Color { r: 0.2, g: 0.5, b: 0.5 }));
    }
}
