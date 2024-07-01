use std::fs::File;
use std::io::{Write, Result};
use crate::color::{Color, color};

#[derive(Debug)]
pub struct Canvas {
    pub height: usize,
    pub width: usize,
    pub pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn equals(&self, c: Canvas) -> bool {
        ( self.width == c.width ) &&
        ( self.height == c.height ) &&
        c_equals(&self.pixels, &c.pixels)
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color) -> () {
        self.pixels[y][x] = c;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y][x]
    }

    pub fn to_ppm(&self, name: &str) -> Result<File> {
        let mut f = File::create(name)?;
        let _ = write!(f, "P3\n");
        let _ = write!(f, "{} {}\n", self.width, self.height);
        let _ = write!(f, "255\n");
        for i in 0..self.pixels.len(){
            let _ = write!(f, "{}", Self::pixel_row_to_string(&self.pixels[i]));
            let _ = write!(f, "\n");
        }
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

    pub fn pixel_to_string(p: Color) -> String {
        let s = format!("{} {} {}",
                        Self::pix_255(p.r),
                        Self::pix_255(p.g),
                        Self::pix_255(p.b));
        s
    }

    pub fn pixel_row_to_string(row: &Vec<Color>) -> String {
        let mut s = String::new();
        let mut index = 0;
        let mut line_length = 0;

        for pixel in row {
            let new_pix = &Self::pixel_to_string(*pixel);
            line_length += new_pix.len() + 1;

            s += new_pix;
            s += " ";

            // PPM line length should not exceed 70 chars
            // maximum pixel string is 12 chars, so 70-12=58
            if index < row.len()-1 && line_length > 58 {
                s += "\n";
                line_length = 0;
            }
            
            index += 1;
        }
        s.to_string()
    }
}

pub fn canvas(width: usize, height:usize) -> Canvas {
    let v = vec![vec![color( 0.0, 0.0, 0.0 );width];height];

    Canvas { width,
             height,
             pixels: v }
}

// uncertain whether this should go into the equals module in
// lib.rs, but for now keeping it here - only Canvas uses it right now
pub fn c_equals(a: &Vec<Vec<Color>>, b: &Vec<Vec<Color>>) -> bool {
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
    use crate::canvas::{Canvas, canvas};
    use crate::color::color;
    use std::fs::read_to_string;

    #[test]
    fn canvas_has_width_and_height(){
        let c = Canvas { width: 10, height: 20, pixels: vec![vec![color( 0.0, 0.0, 0.0 )]] };
        assert!( (c.width == 10) && (c.height == 20) );
    }

    #[test]
    fn canvas_1_1_has_1_pixel(){
        let c = Canvas { width: 1, height: 1, pixels: vec![vec![color( 0.0, 0.0, 0.0 )]] };
        assert!( (c.pixels.len() == 1) && (c.pixels[0].len() == 1) );
    }

    #[test]
    fn canvas_default_color_eq_0_0_0(){
        let c = Canvas { width: 1, height: 1, pixels: vec![vec![color( 0.0, 0.0, 0.0 )]] };
        assert!( c.pixels[0][0].equals( color( 0.0, 0.0, 0.0 ) ));
    }

    #[test]
    fn canvas_created_with_canvas(){
        let c = canvas(1,1);
        assert!( c.equals( Canvas { width: 1, 
                                    height: 1, 
                                    pixels: vec![vec![color( 0.0, 0.0, 0.0 )]] }));
    }

    #[test]
    fn canvas_created_with_canvas_has_pixel_array(){
        let c = canvas(10,20);
        assert!( c.pixels.len() == 20 && c.pixels[0].len() == 10 );
    }

    #[test]
    fn writing_pixels_to_canvas(){
        let mut c = canvas(10,20);
        let red = color( 1.0, 0.0, 0.0 );
        c.write_pixel(2, 3, red);
        assert!( c.pixel_at(2, 3).equals( red ));
    }

    #[test]
    fn constructing_ppm_header(){
        let c = canvas(5,3);
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
        let c = canvas(1,1);
        let _ = c.to_ppm("one_pixel.ppm");
        let lines = read_lines("one_pixel.ppm");
        assert_eq!("0 0 0 ", lines[3]);
    }

    #[test]
    fn ppm_for_color(){
        let mut c = canvas(1,1);
        let red = color( 1.0, 0.0, 0.0 );
        c.write_pixel(0, 0, red);
        let _ = c.to_ppm("red_pixel.ppm");
        let lines = read_lines("red_pixel.ppm");
        assert_eq!("255 0 0 ", lines[3]);
    }

    #[test]
    fn pixel_to_string(){
        let black = color( 0.0, 0.0, 0.0 );
        assert_eq!("0 0 0", Canvas::pixel_to_string(black));
        let red = color( 1.0, 0.0, 0.0 );
        assert_eq!("255 0 0", Canvas::pixel_to_string(red));
        let green = color( 0.0, 1.0, 0.0 );
        assert_eq!("0 255 0", Canvas::pixel_to_string(green));
        let blue = color( 0.0, 0.0, 1.0 );
        assert_eq!("0 0 255", Canvas::pixel_to_string(blue));
        let grey = color( 0.5, 0.5, 0.5 );
        assert_eq!("128 128 128", Canvas::pixel_to_string(grey));
    }
    
    #[test]
    fn ppm_one_row_of_array(){
        let c = canvas(10,10);
        let _ = c.to_ppm("hundred_pixels.ppm");
        let lines = read_lines("hundred_pixels.ppm");
        assert_eq!("0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 ", lines[3]);
    }

    #[test]
    fn ppm_long_row(){
        let c = canvas(25,10);
        let _ = c.to_ppm("long_row.ppm");
        let lines = read_lines("long_row.ppm");
        assert_eq!("0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 ", lines[3]);
    }

    #[test]
    fn ppm_all_pixel_rows(){
        let c = canvas(5,5);
        let _ = c.to_ppm("all_rows.ppm");
        let lines = read_lines("all_rows.ppm");
        assert_eq!(lines.len(),8);
        for n in 3..lines.len() {
            assert_eq!("0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 ", lines[n]);
        }
    }

    #[test]
    fn ppm_coloring_pixels(){
        let mut c = canvas(5, 3);
        let c1 = color( 1.5, 0.0, 0.0 );
        let c2 = color( 0.0, 0.5, 0.0 );
        let c3 = color( -0.5, 0.0, 1.0 );
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        let _ = c.to_ppm("coloring_pixels.ppm");
        let lines = read_lines("coloring_pixels.ppm");
        let expected = ["255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 ",
                        "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 ",
                        "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 "];
        for n in 3..lines.len() {
            assert_eq!(expected[n-3], lines[n]);
        }
    }

    // I'm taking a slightly different approach than the text.
    // I am splitting lines at pixel boundaries, while he
    // seems to be taking the whitespace closest to the 
    // 70-char boundary - my approach is still valid PPM
    // so all good. Test below altered slightly to reflect this.
    #[test]
    fn ppm_split_lines(){
        let w = 9;
        let h = 2;
        let mut c = canvas(w,h);
        let c1 = color( 1.0, 0.8, 0.6 );
        for i in 0..w {
            for j in 0..h {
                c.write_pixel(i,j,c1);   // should implement Canvas.fill()
            }
        }
        let _ = c.to_ppm("split_lines.ppm");
        let lines = read_lines("split_lines.ppm");
        let expected = ["255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 ",
                        "255 204 153 255 204 153 255 204 153 255 204 153 ", 
                        "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 ",
                        "255 204 153 255 204 153 255 204 153 255 204 153 "];
        for n in 3..lines.len() {
            assert_eq!(expected[n-3], lines[n]);
        }
    }
    
    #[test]
    fn ppm_terminated_by_newline(){
        let c = canvas(2,2);
        let _ = c.to_ppm("terminator.ppm");
        let bytes = std::fs::read("terminator.ppm").unwrap();
        let last_byte = bytes[bytes.len()-1];
        assert_eq!('\n', last_byte as char);  
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
