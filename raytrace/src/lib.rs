// note - should use an epsilon value for comparing two floats
//
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_with_w_1_is_point(){
        let a = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 1.0 };
        assert!(
            a.x == 4.3 &&
            a.y == -4.2 &&
            a.z == 3.1 &&
            a.w == 1.0 &&
            a.is_point() &&
            !a.is_vector()
        );
    }

    #[test]
    fn tuple_with_w_0_is_vector(){
        let a = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 0.0 };
        assert!(
            a.x == 4.3 &&
            a.y == -4.2 &&
            a.z == 3.1 &&
            a.w == 0.0 &&
            !a.is_point() &&
            a.is_vector()
        );
    }
}
