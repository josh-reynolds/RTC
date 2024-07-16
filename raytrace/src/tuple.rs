use crate::equals;
use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, t: Self) -> Self {
        Self { x: self.x + t.x,
               y: self.y + t.y,
               z: self.z + t.z,
               w: self.w + t.w }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, t: Self) -> Self {
        Self { x: self.x - t.x,
               y: self.y - t.y,
               z: self.z - t.z,
               w: self.w - t.w }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self { x: self.x * rhs,
               y: self.y * rhs,
               z: self.z * rhs,
               w: self.w * rhs }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, n: f64) -> Self {
        Self { x: self.x / n,
               y: self.y / n,
               z: self.z / n,
               w: self.w / n }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { x: -self.x,
               y: -self.y,
               z: -self.z,
               w: -self.w }
    }
}

impl Tuple {
    pub fn is_point(&self) -> bool {
        equals( self.w, 1.0 )
    }

    pub fn is_vector(&self) -> bool {
        equals( self.w, 0.0 )
    }

    // only applies to vectors - should we restrict usage?
    pub fn mag(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    // also only applies to vectors
    pub fn normal(&self) -> Self {
        let mag = self.mag();
        Self { x: self.x / mag,
               y: self.y / mag,
               z: self.z / mag,
               w: 0.0}
    }

    pub fn dot(&self, v: &Tuple) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z + self.w + v.w
    }

    // also only applies to vectors
    pub fn cross(&self, v: &Tuple) -> Self {
        Self { x: self.y * v.z - self.z * v.y,
               y: self.z * v.x - self.x * v.z,
               z: self.x * v.y - self.y * v.x,
               w: 0.0 }
    }

    // also only applies to vectors
    pub fn reflect(&self, v: &Tuple) -> Self {
        *self - (*v * 2.0 * self.dot(&v))
    }

    pub fn equals(&self, t: Tuple) -> bool {
        equals( self.x, t.x ) && 
        equals( self.y, t.y ) &&
        equals( self.z, t.z ) && 
        equals( self.w, t.w )
    }
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

pub fn origin() -> Tuple {
    Tuple { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
}

#[cfg(test)]
mod tests {
    use crate::equals;
    use crate::tuple::{Tuple, point, vector, origin};
    use std::f64::consts::SQRT_2;

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
        let a = point(4.0, -4.0, 3.0);
        assert!( a.equals(Tuple { x: 4.0, y: -4.0, z: 3.0, w: 1.0 }));
    }

    #[test]
    fn origin_creates_point_at_origin(){
        let o = origin();
        assert!( o.equals(Tuple { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }));
    }

    #[test]
    fn vector_creates_vectors(){
        let a = vector(4.0, -4.0, 3.0);
        assert!( a.equals(Tuple { x: 4.0, y: -4.0, z: 3.0, w: 0.0 }));

    }

    // Going to break the next two tests at least temporarily while
    // we remove the Number type that enabled this scenario - it
    // is cluttering up the code. Long term is to figure out a 
    // solution using generics. For now we'll only allow creation
    // from floats.
//    #[test]
//    fn can_create_points_from_ints(){
//        let a = Tuple::point(Number::from(4),
//                             Number::from(-4),
//                             Number::from(3));
//        assert!( a.equals(Tuple { x: 4.0, y: -4.0, z: 3.0, w: 1.0 }));
//    }

//    #[test]
//    fn can_create_vectors_from_ints(){
//        let a = Tuple::vector(Number::from(4),
//                              Number::from(-4),
//                              Number::from(3));
//        assert!( a.equals(Tuple { x: 4.0, y: -4.0, z: 3.0, w: 0.0 }));
//    }

    // OK: vector + vector, vector + point
    // Not OK: point + point
    #[test]
    fn add_two_tuples(){
        let a = point(3.0, -2.0, 5.0);
        let b = vector(-2.0, 3.0, 1.0);
        assert!( (a + b).equals(Tuple { x: 1.0, y: 1.0, z: 6.0, w: 1.0 }));
    }

    #[test]
    fn sub_two_points(){
        let a = point(3.0, 2.0, 1.0);
        let b = point(5.0, 6.0, 7.0);
        assert!( (a - b).equals(vector(-2.0, -4.0, -6.0)));
    }

    #[test]
    fn sub_vector_from_point(){
        let a = point(3.0, 2.0, 1.0);
        let b = vector(5.0, 6.0, 7.0);
        assert!( (a - b).equals(point(-2.0, -4.0, -6.0)));
    }
    
    #[test]
    fn sub_two_vectors(){
        let a = vector(3.0, 2.0, 1.0);
        let b = vector(5.0, 6.0, 7.0);
        assert!( (a - b).equals(vector(-2.0, -4.0, -6.0)));
    }

    #[test]
    fn sub_vector_from_zero_vector(){
        let zero = vector(0.0, 0.0, 0.0);
        let v = vector(1.0, -2.0, 3.0);
        assert!( (zero - v).equals(vector(-1.0, 2.0, -3.0)));
    }

    #[test]
    fn negating_a_tuple(){
        let a = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        assert!( (-a).equals(Tuple { x: -1.0, y: 2.0, z: -3.0, w: 4.0 } ));
    }

    #[test]
    fn multipy_tuple_by_scalar(){
        let a = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        assert!( (a * 3.5).equals( Tuple { x: 3.5, y: -7.0, z: 10.5, w: -14.0 } ));
    }

    #[test]
    fn multipy_tuple_by_fraction(){
        let a = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        assert!( (a * 0.5).equals( Tuple { x: 0.5, y: -1.0, z: 1.5, w: -2.0 } ));
    }

    #[test]
    fn divide_tuple_by_scalar(){
        let a = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        assert!( (a / 2.0).equals( Tuple { x: 0.5, y: -1.0, z: 1.5, w: -2.0 } ));
    }

    #[test]
    fn magnitude_of_vector_1_0_0(){
        let v = vector(1.0, 0.0, 0.0);
        assert!( equals( v.mag(), 1.0 ));
    }

    #[test]
    fn magnitude_of_vector_0_1_0(){
        let v = vector(0.0, 1.0, 0.0);
        assert!( equals( v.mag(), 1.0 ));
    }

    #[test]
    fn magnitude_of_vector_0_0_1(){
        let v = vector(0.0, 0.0, 1.0);
        assert!( equals( v.mag(), 1.0 ));
    }

    #[test]
    fn magnitude_of_vector_1_2_3(){
        let v = vector(1.0, 2.0, 3.0);
        assert_eq!( 14_f64.sqrt(), v.mag() );
    }

    #[test]
    fn magnitude_of_neg_vector_1_2_3(){
        let v = vector(-1.0, -2.0, -3.0);
        assert_eq!( 14_f64.sqrt(), v.mag() );
    }

    #[test]
    fn normalize_vector_4_0_0_equals_vector_1_0_0(){
        let v = vector(4.0, 0.0, 0.0);
        assert!( v.normal().equals( Tuple { x: 1.0, y: 0.0, z: 0.0, w: 0.0 } ));
    }

    #[test]
    fn normalize_vector_1_2_3(){
        let v = vector(1.0, 2.0, 3.0);
        assert!( v.normal().equals( Tuple { x: 1.0 / 14_f64.sqrt(), y: 2.0 / 14_f64.sqrt(),
                                            z: 3.0 / 14_f64.sqrt(), w: 0.0 } ));
    }

    #[test]
    fn normalized_vector_mag_equals_1(){
        let v = vector(1.0, 2.0, 3.0);
        assert_eq!( v.normal().mag(), 1.0 );
    }

    #[test]
    fn dot_product_1_2_3_w_2_3_4_equals_20(){
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert_eq!( a.dot(&b), 20.0 );
    }

    #[test]
    fn cross_product_1_2_3_w_2_3_4_equals_neg1_2_neg1(){
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert!( a.cross(&b).equals( vector(-1.0, 2.0, -1.0)));
        assert!( b.cross(&a).equals( vector(1.0, -2.0, 1.0)));
    }    

    #[test]
    fn reflecting_vector_approaching_at_45_deg(){
        let v = vector(1.0, -1.0, 0.0);
        let n = vector(0.0, 1.0, 0.0);
        let r = v.reflect(&n);
        assert!( r.equals( vector(1.0, 1.0, 0.0)));
    }

    #[test]
    fn reflecting_vector_off_slanted_surface(){
        let v = vector(0.0, -1.0, 0.0);
        let n = vector(SQRT_2 / 2.0, SQRT_2 / 2.0, 0.0);
        let r = v.reflect(&n);
        assert!( r.equals( vector(1.0, 0.0, 0.0)));
    }
}
