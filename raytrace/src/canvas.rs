use crate::color::Color;

#[derive(Debug)]
pub struct Canvas {
    pub height: i32,
    pub width: i32,
    pub pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: i32, height:i32) -> Self {
        Self { width: width,
               height: height,
               pixels: vec![vec![Color{r:0.0,g:0.0,b:0.0}]] }
    }

    pub fn equals(&self, c: Canvas) -> bool {
        ( self.width == c.width ) &&
        ( self.height == c.height ) &&
        false
        //( self.pixels.equals(c.pixels) ) // need to implement pixel array comparison
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::color::Color;

    #[test]
    fn canvas_has_width_and_height(){
        let c = Canvas { width: 10, height: 20, pixels: vec![vec![Color{r:0.0,g:0.0,b:0.0}]] };
        assert!( (c.width == 10) && (c.height == 20) );
    }

    #[test]
    fn canvas_1_1_has_1_pixel(){
        let c = Canvas { width: 1, height: 1, pixels: vec![vec![Color{r:0.0,g:0.0,b:0.0}]] };
        assert!( (c.pixels.len() == 1) && (c.pixels[0].len() == 1) );
    }

    #[test]
    fn canvas_default_color_eq_0_0_0(){
        let c = Canvas { width: 1, height: 1, pixels: vec![vec![Color{r:0.0,g:0.0,b:0.0}]] };
        assert!( c.pixels[0][0].equals( Color{r:0.0,g:0.0,b:0.0} ));
    }

    #[test]
    fn canvas_created_with_new(){
        let c = Canvas::new(1,1);
        assert!( c.equals( Canvas { width: 1, 
                                    height: 1, 
                                    pixels: vec![vec![Color{r:0.0,g:0.0,b:0.0}]] }));
    }
}
