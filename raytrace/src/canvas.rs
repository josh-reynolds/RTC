use crate::color::Color;

#[derive(Debug)]
pub struct Canvas {
    pub height: usize,
    pub width: usize,
    pub pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height:usize) -> Self {
        let v = vec![vec![Color{r:0.0,g:0.0,b:0.0};width];height];

        Self { width: width,
               height: height,
               pixels: v }
    }

    pub fn equals(&self, c: Canvas) -> bool {
        ( self.width == c.width ) &&
        ( self.height == c.height ) &&
        equals(&self.pixels, &c.pixels)
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color) -> () {
        self.pixels[x][y] = c;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
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
    fn canvas_created_with_new_has_pixel_array(){
        let c = Canvas::new(10,20);
        assert!( c.pixels.len() == 20 && c.pixels[0].len() == 10 );
    }

    #[test]
    fn writing_pixels_to_canvas(){
        let mut c = Canvas::new(10,20);
        let red = Color{r:1.0,g:0.0,b:0.0};
        c.write_pixel(2, 3, red);

        println!( "{:?}", c.pixel_at(2, 3) );

        assert!( c.pixel_at(2, 3).equals( red ));
    }
}
