use crate::matrix::{Matrix, identity};

#[derive(Debug)]
pub struct Shape {           // Shape should probably be a trait, not a struct
                             // will be moving in that direction
    pub transform: Matrix,
}

pub fn test_shape() -> Shape {
    Shape {
        transform: identity(),
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::identity;
    use crate::shapes::test_shape;

    #[test]
    fn shape_default_transform(){
        let s = test_shape();
        assert!( s.transform.equals( identity() ));
    }
}
