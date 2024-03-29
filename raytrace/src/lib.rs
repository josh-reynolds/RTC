use crate::equals::equals;

pub mod color;
pub mod number;
pub mod tuple;

mod equals {
    const EPSILON: f64 = 0.00001;
    
    pub fn equals(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON 
    }
}

#[cfg(test)]
mod tests {
    use crate::equals;

    #[test]
    fn float_equals(){
        assert!( equals(1.0, 1.0) );
    }

    #[test]
    fn float_not_equals(){
        assert!( !equals(1.0, 1.001) );
    }
}
