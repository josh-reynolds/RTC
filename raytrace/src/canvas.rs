use std::fs::File;
use std::io::{Write};
use std::io::Result;
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

    pub fn to_ppm(&self, name: &str) -> Result<File> {
        let mut f = File::create(name)?;
        let _ = write!(f, "P3\n");
        let _ = write!(f, "{} {}\n", self.width, self.height);
        let _ = write!(f, "255\n");
        let _ = write!(f, "0 0 0");
        Ok(f)
    }

    // test generates a cargo warning unless this is marked
    // public - probably should be an internal-only fn though
    pub fn pix_255(value: f64) -> i32 {
        let mut result = (value * 256.0).floor() as i32;
        result = if result > 255 { 255 } else { result };
        result = if result <   0 {   0 } else { result };
        result
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
    use std::fs::read_to_string;

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
        assert!( c.pixel_at(2, 3).equals( red ));
    }

    #[test]
    fn constructing_ppm_header(){
        let c = Canvas::new(5,3);
        let _ = c.to_ppm("header.ppm");

        let result = ["P3", "5 3", "255"];
        let lines = read_lines("header.ppm");

        for i in 0..3{
            assert_eq!(result[i], lines[i]);
        }
    }

    #[test]
    fn pixel_value_to_255_scale(){
        let a = 1.0;
        assert!( Canvas::pix_255(a) == 255 );

        let b = 0.5;
        assert!( Canvas::pix_255(b) == 128 );

        let c = 0.0;
        assert!( Canvas::pix_255(c) == 0 );

        let d = 1.5;
        assert!( Canvas::pix_255(d) == 255 );

        let e = -0.5;
        assert!( Canvas::pix_255(e) == 0 );
    }

    #[test]
    fn ppm_for_one_pixel(){
        let c = Canvas::new(1,1);
        let _ = c.to_ppm("one_pixel.ppm");
        let lines = read_lines("one_pixel.ppm");
        assert_eq!("0 0 0", lines[3]);
    }

    // leaving this as test helper function for now
    // will probably have utility elsewhere and be moved later
    fn read_lines(filename: &str) -> Vec<String> {
        let mut result = Vec::new();

        for line in read_to_string(filename).unwrap().lines() {
            result.push(line.to_string())
        }

        result
    }


}
