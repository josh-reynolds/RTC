use std::convert::From;

pub struct Number {
    pub value: f64,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number{ value: item as f64 }
    }
}

impl From<f64> for Number {
    fn from(item: f64) -> Self {
        Number{ value: item }
    }
}


#[cfg(test)]
mod tests {
    use crate::equals;
    use crate::number::Number;

    #[test]
    fn number_from_float(){
        let n = Number::from( 1.0 );
        assert!( equals(n.value, 1.0) );
    }

    #[test]
    fn number_from_int(){
        let n = Number::from( 1 );
        assert!( equals(n.value, 1.0) );
    }
}
