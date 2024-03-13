use std::convert::From;

const EPSILON: f64 = 0.00001;

pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn is_point(&self) -> bool {
        equals( self.w, 1.0 )
    }

    pub fn is_vector(&self) -> bool {
        equals( self.w, 0.0 )
    }

    pub fn point(x: Number, y: Number, z: Number) -> Self {
        Self { x: x.value, y: y.value, z: z.value, w: 1.0 }
    }

    pub fn vector(x: Number, y: Number, z: Number) -> Self {
        Self { x: x.value, y: y.value, z: z.value, w: 0.0 }
    }

    pub fn plus(&self, t: Tuple) -> Self {
        Self { x: self.x + t.x,
               y: self.y + t.y,
               z: self.z + t.z,
               w: self.w + t.w }
    }

    pub fn sub(&self, t: Tuple) -> Self {
        Self { x: self.x - t.x,
               y: self.y - t.y,
               z: self.z - t.z,
               w: self.w - t.w }
    }

    // investigate operator overloading, appears to be in std::ops
    // the trait needed is Neg
    pub fn negate(&self) -> Self {
        Self { x: -self.x,
               y: -self.y,
               z: -self.z,
               w: -self.w }
    }

    // overloading possible here too (trait == Mul)
    // also, how about multiplying by an int? Number again?
    pub fn mult(&self, n: f64) -> Self {
        Self { x: self.x * n,
               y: self.y * n,
               z: self.z * n,
               w: self.w * n }
    }

    pub fn equals(&self, t: Tuple) -> bool {
        equals( self.x, t.x ) && 
        equals( self.y, t.y ) &&
        equals( self.z, t.z ) && 
        equals( self.w, t.w )
    }
}

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

pub fn equals(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_with_w_1_is_point(){
        let a = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 1.0 };
        assert!(
            equals( a.x, 4.3 ) &&
            equals( a.y, -4.2 ) &&
            equals( a.z, 3.1 ) &&
            equals( a.w, 1.0 ) &&
            a.is_point() &&
            !a.is_vector()
        );
    }

    #[test]
    fn tuple_with_w_0_is_vector(){
        let a = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 0.0 };
        assert!(
            equals( a.x, 4.3 ) &&
            equals( a.y, -4.2 ) &&
            equals( a.z, 3.1 ) &&
            equals( a.w, 0.0 ) &&
            !a.is_point() &&
            a.is_vector()
        );
    }

    #[test]
    fn point_creates_points(){
        let a = Tuple::point(Number::from(4.0), 
                             Number::from(-4.0),
                             Number::from(3.0));
        assert!( a.equals(Tuple { x: 4.0, y: -4.0, z: 3.0, w: 1.0 }));
    }

    #[test]
    fn vector_creates_vectors(){
        let a = Tuple::vector(Number::from(4.0),
                              Number::from(-4.0),
                              Number::from(3.0));
        assert!( a.equals(Tuple { x: 4.0, y: -4.0, z: 3.0, w: 0.0 }));

    }

    #[test]
    fn float_equals(){
        assert!( equals(1.0, 1.0) );
    }

    #[test]
    fn float_not_equals(){
        assert!( !equals(1.0, 1.001) );
    }

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

    #[test]
    fn can_create_points_from_ints(){
        let a = Tuple::point(Number::from(4),
                             Number::from(-4),
                             Number::from(3));
        assert!( a.equals(Tuple { x: 4.0, y: -4.0, z: 3.0, w: 1.0 }));
    }

    #[test]
    fn can_create_vectors_from_ints(){
        let a = Tuple::vector(Number::from(4),
                              Number::from(-4),
                              Number::from(3));
        assert!( a.equals(Tuple { x: 4.0, y: -4.0, z: 3.0, w: 0.0 }));
    }

    // OK: vector + vector, vector + point
    // Not OK: point + point
    #[test]
    fn add_two_tuples(){
        let a = Tuple::point(Number::from(3),
                             Number::from(-2),
                             Number::from(5));
        let b = Tuple::vector(Number::from(-2),
                              Number::from(3),
                              Number::from(1));
        assert!( a.plus(b).equals(Tuple { x: 1.0, y: 1.0, z: 6.0, w: 1.0 }));
    }

    #[test]
    fn sub_two_points(){
        let a = Tuple::point(Number::from(3),
                             Number::from(2),
                             Number::from(1));
        let b = Tuple::point(Number::from(5),
                             Number::from(6),
                             Number::from(7));
        assert!( a.sub(b).equals(Tuple::vector(Number::from(-2.0), 
                                               Number::from(-4.0), 
                                               Number::from(-6.0))));
    }

    #[test]
    fn sub_vector_from_point(){
        let a = Tuple::point(Number::from(3),
                             Number::from(2),
                             Number::from(1));
        let b = Tuple::vector(Number::from(5),
                              Number::from(6),
                              Number::from(7));
        assert!( a.sub(b).equals(Tuple::point(Number::from(-2.0), 
                                              Number::from(-4.0), 
                                              Number::from(-6.0))));
    }
    
    #[test]
    fn sub_two_vectors(){
        let a = Tuple::vector(Number::from(3),
                              Number::from(2),
                              Number::from(1));
        let b = Tuple::vector(Number::from(5),
                              Number::from(6),
                              Number::from(7));
        assert!( a.sub(b).equals(Tuple::vector(Number::from(-2.0), 
                                               Number::from(-4.0), 
                                               Number::from(-6.0))));
    }

    #[test]
    fn sub_vector_from_zero_vector(){
        let zero = Tuple::vector(Number::from(0),
                                 Number::from(0),
                                 Number::from(0));
        let v = Tuple::vector(Number::from(1),
                              Number::from(-2),
                              Number::from(3));
        assert!( zero.sub(v).equals(Tuple::vector(Number::from(-1.0), 
                                                  Number::from(2.0), 
                                                  Number::from(-3.0))));
    }

    #[test]
    fn negating_a_tuple(){
        let a = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        assert!( a.negate().equals(Tuple { x: -1.0, y: 2.0, z: -3.0, w: 4.0 } ));
    }

    #[test]
    fn multipy_tuple_by_scalar(){
        let a = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        assert!( a.mult(3.5).equals( Tuple { x: 3.5, y: -7.0, z: 10.5, w: -14.0 } ));
    }

    #[test]
    fn multipy_tuple_by_fraction(){
        let a = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        assert!( a.mult(0.5).equals( Tuple { x: 0.5, y: -1.0, z: 1.5, w: -2.0 } ));
    }
}
