
#[derive(Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
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
}
