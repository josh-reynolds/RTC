use crate::equals::equals;

pub mod canvas;
pub mod color;
pub mod tuple;
pub mod matrix;
pub mod transform;
pub mod rays;
pub mod spheres;
pub mod intersections;
pub mod lights;
pub mod materials;
pub mod world;
pub mod camera;
pub mod shapes;

mod equals {
    pub const EPSILON: f64 = 0.00001;
    
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
