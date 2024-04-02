use crate::color::Color;

#[derive(Debug)]
pub struct Canvas {
    pub height: i32,
    pub width: i32,
    pub pixels: Vec<Vec<Color>>,
}


#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::color::Color;

    #[test]
    fn canvas_has_width_and_height(){
        let c = Canvas { width: 10, height: 20, pixels: vec![vec![Color{r:0.0,g:0.0,b:0.0}]]};
        assert!( (c.width == 10) && (c.height == 20) );
    }

    #[test]
    fn canvas_1_1_has_1_pixel(){
        let c = Canvas { width: 1, height: 1, pixels: vec![vec![Color{r:0.0,g:0.0,b:0.0}]] };

    }
}
