
#[derive(Debug)]
pub struct Canvas {
    pub height: i32,
    pub width: i32,
}


#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;

    #[test]
    fn canvas_has_width_and_height(){
        let c = Canvas { width: 10, height: 20 };
        assert!( (c.width == 10) && (c.height == 20) );
    }
}
