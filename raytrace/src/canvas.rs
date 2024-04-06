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
        equals(&self.pixels, &c.pixels)
    }

    pub fn write_pixel(&self, x: usize, y: usize, c: Color) -> () {
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        println!("{} {}", self.width, self.height);
        println!("{:?}", self.pixels);
        self.pixels[x][y]
    }
}

// uncertain whether this should go into the equals module in
// lib.rs, but for now keeping it here - only Canvas uses it right now
pub fn equals(a: &Vec<Vec<Color>>, b: &Vec<Vec<Color>>) -> bool {
    if (a.len() != b.len()) || (a[0].len() != b[0].len()){
        return false
    }
    
    for n in 0..(a.len()) {
        for m in 0..(a[n].len()){
            if !a[n][m].equals(b[n][m]){
                return false
            }
        }
    }

    true
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

    #[test]
    fn writing_pixels_to_canvas(){
        let c = Canvas::new(10,20);
        let red = Color{r:1.0,g:0.0,b:0.0};
        c.write_pixel(2, 3, red);
        assert!( c.pixel_at(2, 3).equals( red ));
    }
}
