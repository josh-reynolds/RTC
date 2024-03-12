const EPSILON: f64 = 0.001;

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

    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 1.0 }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn equals(&self, t: Tuple) -> bool {
        self.x == t.x && self.y == t.y &&
        self.z == t.z && self.w == t.w
    }
}

pub fn equals(a: f64, b: f64) -> bool {
    (a - b).abs() <= EPSILON 
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

    #[test]
    fn point_creates_points(){
        // should allow integers as well
        let a = Tuple::point(4.0, -4.0, 3.0);
        assert!( a.equals(Tuple { x: 4.0, y: -4.0, z: 3.0, w: 1.0 }));
    }

    #[test]
    fn vector_creates_vectors(){
        // should allow integers as well
        let a = Tuple::vector(4.0, -4.0, 3.0);
    }
}
