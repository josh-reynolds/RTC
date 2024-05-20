use crate::equals;
use crate::tuple::Tuple;

#[derive(Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub m: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self { cols: cols,
               rows: rows,
               m: vec![vec![0.0;cols];rows] }
    }

    pub fn identity() -> Self {
        Self { cols: 4,
               rows: 4,
               m: vec![vec![1.0, 0.0, 0.0, 0.0],
                       vec![0.0, 1.0, 0.0, 0.0],
                       vec![0.0, 0.0, 1.0, 0.0],
                       vec![0.0, 0.0, 0.0, 1.0]] }
    }

    pub fn equals(&self, m: Matrix) -> bool {
        ( self.rows == m.rows ) &&
        ( self.cols == m.cols ) &&
        v_equals(&self.m, &m.m )
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.m[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, val: f64) {
        self.m[row][col] = val;
    }

    // only valid for 4x4 matrices - should add assert
    pub fn mult(&self, other: Matrix) -> Self {
        let mut m = Matrix::new(4,4);
        for row in 0..4 {
            for col in 0..4 {
                let val = self.get(row,0) * other.get(0,col) +
                          self.get(row,1) * other.get(1,col) +
                          self.get(row,2) * other.get(2,col) +
                          self.get(row,3) * other.get(3,col);
                m.set(row,col,val);
            }
        }
        m
    }

    pub fn multup(&self, other: &Tuple) -> Tuple {
        let mut t = Tuple { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };

        t.x = self.get(0,0) * other.x +
              self.get(0,1) * other.y +
              self.get(0,2) * other.z +
              self.get(0,3) * other.w;
            
        t.y = self.get(1,0) * other.x +
              self.get(1,1) * other.y +
              self.get(1,2) * other.z +
              self.get(1,3) * other.w;
        
        t.z = self.get(2,0) * other.x +
              self.get(2,1) * other.y +
              self.get(2,2) * other.z +
              self.get(2,3) * other.w;

        t.w = self.get(3,0) * other.x +
              self.get(3,1) * other.y +
              self.get(3,2) * other.z +
              self.get(3,3) * other.w;

        t
    }

    pub fn transpose(&self) -> Self {
        let mut m = Matrix::new(4,4);
        for row in 0..4 {
            for col in 0..4 {
                let val = self.get(col,row);
                m.set(row,col,val);
            }
        }
        m
    }

    // adding logic for larger sizes...
    // assumes square matrices...
    pub fn determinant(&self) -> f64 {
        let mut det = 0.0;
        let size = self.cols;

        if size == 2 {
            det = (self.get(0,0) * self.get(1,1)) - 
                  (self.get(1,0) * self.get(0,1));
        } else {
            for col in 0..size {
                det = det + self.get(0,col) * self.cofactor(0,col);
            }
        }

        det
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Self {
        let mut m = Matrix::new(self.rows-1, self.cols-1);

        let mut current_row = 0;
        for r in 0..self.rows {
            if r == row { continue };

            let mut current_col = 0;
            for c in 0..self.cols {
                if c == col { continue };

                m.set(current_row,current_col,self.get(r,c));
                current_col += 1;
            }
            current_row += 1;
        }
        m
    }

    // only valid for 3x3 matrices - should add assert
    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row,col).determinant()
    }

    // only valid for 3x3 matrices - should add assert
    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let mut result = self.minor(row,col);

        if (row + col) % 2 != 0 {
            result = -result;
        }

        result
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Self {
        if !self.is_invertible(){
            panic!();
        }

        let size = self.cols;
        let det = self.determinant();
        let mut result = Matrix::new(size, size);

        for row in 0..size {
            for col in 0..size {
                let c = self.cofactor(row, col);
                // swapping row & col below because we
                // want a transposed matrix
                result.set(col, row, c / det);
            }
        }
        result
    }
}

// very similar function in Canvas, may want to refactor all
// this later
pub fn v_equals(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> bool {
    if (a.len() != b.len()) || (a[0].len() != b[0].len()){
        return false
    }

    for n in 0..(a.len()) {
        for m in 0..(a[n].len()){
            if !equals(a[n][m], b[n][m]){
                return false
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;
    use crate::number::Number;
    use crate::tuple::Tuple;

    #[test]
    fn matrix_has_cols_and_rows(){
        let m = Matrix { cols: 4, rows: 4, m: vec![vec![0.0]] };
        assert!( (m.cols == 4) && (m.rows == 4) );
    }

    #[test]
    fn matrix_created_with_new(){
        let m = Matrix::new(4,4);
        assert!( m.equals( Matrix { cols: 4,
                                    rows: 4,
                                    m: vec![vec![0.0;4];4] }));
    }

    #[test]
    fn matrix_inspection(){
        let m = Matrix { cols: 4, rows: 4,
                         m: vec![vec![1.0,2.0,3.0,4.0],
                                 vec![5.5,6.5,7.5,8.5],
                                 vec![9.0,10.0,11.0,12.0],
                                 vec![13.5,14.5,15.5,16.5]] };
        assert!( m.m[0][0] == 1.0 &&
                 m.m[0][3] == 4.0 &&
                 m.m[1][0] == 5.5 &&
                 m.m[1][2] == 7.5 &&
                 m.m[2][2] == 11.0 &&
                 m.m[3][0] == 13.5 &&
                 m.m[3][2] == 15.5 );
    }

    #[test]
    fn two_by_two_matrix(){
        let m = Matrix { cols: 2, rows: 2,
                         m: vec![vec![-3.0,5.0],
                                 vec![1.0,-2.0]] };
        assert!( m.m[0][0] == -3.0 &&
                 m.m[0][1] == 5.0 &&
                 m.m[1][0] == 1.0 &&
                 m.m[1][1] == -2.0 );
    }

    #[test]
    fn three_by_three_matrix(){
        let m = Matrix { cols: 3, rows: 3,
                         m: vec![vec![-3.0,5.0,0.0],
                                 vec![1.0,-2.0,-7.0],
                                 vec![0.0,1.0,1.0]] };
        assert!( m.m[0][0] == -3.0 &&
                 m.m[1][1] == -2.0 &&
                 m.m[2][2] == 1.0 );
    }

    #[test]
    fn matrix_equality(){
        let a = Matrix { cols: 4, rows: 4,
                         m: vec![vec![1.0,2.0,3.0,4.0],
                                 vec![5.0,6.0,7.0,8.0],
                                 vec![9.0,8.0,7.0,6.0],
                                 vec![5.0,4.0,3.0,2.0]] };

        let b = Matrix { cols: 4, rows: 4,
                         m: vec![vec![1.0,2.0,3.0,4.0],
                                 vec![5.0,6.0,7.0,8.0],
                                 vec![9.0,8.0,7.0,6.0],
                                 vec![5.0,4.0,3.0,2.0]] };
        assert!( a.equals(b) );
    }

    #[test]
    fn matrix_inequality(){
        let a = Matrix { cols: 4, rows: 4,
                         m: vec![vec![1.0,2.0,3.0,4.0],
                                 vec![5.0,6.0,7.0,8.0],
                                 vec![9.0,8.0,7.0,6.0],
                                 vec![5.0,4.0,3.0,2.0]] };

        let b = Matrix { cols: 4, rows: 4,
                         m: vec![vec![2.0,3.0,4.0,5.0],
                                 vec![6.0,7.0,8.0,9.0],
                                 vec![8.0,7.0,6.0,5.0],
                                 vec![4.0,3.0,2.0,1.0]] };
        assert!( !a.equals(b) );
    }

    #[test]
    fn matrix_getter(){
        let m = Matrix { cols: 4, rows: 4,
                         m: vec![vec![1.0,2.0,3.0,4.0],
                                 vec![5.0,6.0,7.0,8.0],
                                 vec![9.0,8.0,7.0,6.0],
                                 vec![5.0,4.0,3.0,2.0]] };
        assert!( m.get(1,1) == 6.0 );
        assert!( m.get(0,2) == 3.0 );
    }

    #[test]
    fn matrix_multiply_4_by_4(){
        let a = Matrix { cols: 4, rows: 4,
                         m: vec![vec![1.0, 2.0, 3.0, 4.0],
                                 vec![5.0, 6.0, 7.0, 8.0],
                                 vec![9.0, 8.0, 7.0, 6.0],
                                 vec![5.0, 4.0, 3.0, 2.0]] };

        let b = Matrix { cols: 4, rows: 4,
                         m: vec![vec![-2.0, 1.0, 2.0,  3.0],
                                 vec![ 3.0, 2.0, 1.0, -1.0],
                                 vec![ 4.0, 3.0, 6.0,  5.0],
                                 vec![ 1.0, 2.0, 7.0, 8.0]] };

        let result = Matrix { cols: 4, rows: 4,
                         m: vec![vec![20.0, 22.0,  50.0,  48.0],
                                 vec![44.0, 54.0, 114.0, 108.0],
                                 vec![40.0, 58.0, 110.0, 102.0],
                                 vec![16.0, 26.0,  46.0,  42.0]] };

        assert!( a.mult(b).equals(result) );
    }

    #[test]
    fn matrix_multiply_4_by_tuple(){
        let a = Matrix { cols: 4, rows: 4,
                         m: vec![vec![1.0, 2.0, 3.0, 4.0],
                                 vec![2.0, 4.0, 4.0, 2.0],
                                 vec![8.0, 6.0, 4.0, 1.0],
                                 vec![0.0, 0.0, 0.0, 1.0]] };

        let b = Tuple::point(Number::from(1.0), 
                             Number::from(2.0),
                             Number::from(3.0));

        assert!( a.multup(&b).equals( Tuple::point(Number::from(18.0), 
                                                  Number::from(24.0),
                                                  Number::from(33.0)) ));
    }

    #[test]
    fn matrix_identity(){
        let identity = Matrix { cols: 4, rows: 4,
                                m: vec![vec![1.0, 0.0, 0.0, 0.0],
                                        vec![0.0, 1.0, 0.0, 0.0],
                                        vec![0.0, 0.0, 1.0, 0.0],
                                        vec![0.0, 0.0, 0.0, 1.0]] };
        
        assert!( Matrix::identity().equals(identity) );
    }

    #[test]
    fn matrix_setter(){
        let mut m = Matrix { cols: 4, rows: 4,
                         m: vec![vec![1.0,2.0,3.0,4.0],
                                 vec![5.0,6.0,7.0,8.0],
                                 vec![9.0,8.0,7.0,6.0],
                                 vec![5.0,4.0,3.0,2.0]] };
        m.set(1,1,12.0);
        assert!( m.get(1,1) == 12.0 );
    }

    #[test]
    fn matrix_multiply_by_identity(){
        let m = Matrix { cols: 4, rows: 4,
                         m: vec![vec![0.0,1.0, 2.0, 4.0],
                                 vec![1.0,2.0, 4.0, 8.0],
                                 vec![2.0,4.0, 8.0,16.0],
                                 vec![4.0,8.0,16.0,32.0]] };
        assert!( m.mult(Matrix::identity()).equals(m));
    }

    #[test]
    fn tuple_multiply_by_identity(){
        let t = Tuple{ x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
        assert!( Matrix::identity().multup(&t).equals(t) );
    }

    #[test]
    fn matrix_transpose(){
        let m = Matrix { cols: 4, rows: 4,
                         m: vec![vec![0.0,9.0,3.0,0.0],
                                 vec![9.0,8.0,0.0,8.0],
                                 vec![1.0,8.0,5.0,3.0],
                                 vec![0.0,0.0,5.0,8.0]] };

        let result = Matrix { cols: 4, rows: 4,
                              m: vec![vec![0.0,9.0,1.0,0.0],
                                      vec![9.0,8.0,8.0,0.0],
                                      vec![3.0,0.0,5.0,5.0],
                                      vec![0.0,8.0,3.0,8.0]] };

        assert!( m.transpose().equals(result) );
    }

    #[test]
    fn matrix_transpose_identity(){
        assert!( Matrix::identity().transpose().equals(Matrix::identity()) );
    }

    #[test]
    fn matrix_determinant_2_by_2(){
        let m = Matrix { cols: 2, rows: 2,
                         m: vec![vec![ 1.0,5.0],
                                 vec![-3.0,2.0]] };

        assert!( m.determinant() == 17.0 );
    }

    #[test]
    fn submatrix_of_3_by_3(){
        let m = Matrix { cols: 3, rows: 3,
                         m: vec![vec![ 1.0,5.0, 0.0],
                                 vec![-3.0,2.0, 7.0],
                                 vec![ 0.0,6.0,-3.0]] };

        let sub = Matrix { cols: 2, rows: 2,
                           m: vec![vec![-3.0,2.0],
                                   vec![ 0.0,6.0]] };

        assert!( m.submatrix(0,2).equals(sub) );
    }

    #[test]
    fn submatrix_of_4_by_4(){
        let m = Matrix { cols: 4, rows: 4,
                         m: vec![vec![-6.0,1.0, 1.0,6.0],
                                 vec![-8.0,5.0, 8.0,6.0],
                                 vec![-1.0,0.0, 8.0,2.0],
                                 vec![-7.0,1.0,-1.0,1.0]] };

        let sub = Matrix { cols: 3, rows: 3,
                           m: vec![vec![-6.0, 1.0,6.0],
                                   vec![-8.0, 8.0,6.0],
                                   vec![-7.0,-1.0,1.0]] };

        assert!( m.submatrix(2,1).equals(sub) );

    }

    #[test]
    fn minor_of_3_by_3(){
        let a = Matrix { cols: 3, rows: 3,
                         m: vec![vec![3.0, 5.0, 0.0],
                                 vec![2.0,-1.0,-7.0],
                                 vec![6.0,-1.0, 5.0]] };

        let b = a.submatrix(1,0);

        assert!( b.determinant() == 25.0 );
        assert!( a.minor(1,0) == 25.0 );
    }

    #[test]
    fn cofactor_of_3_by_3(){
        let a = Matrix { cols: 3, rows: 3,
                         m: vec![vec![3.0, 5.0, 0.0],
                                 vec![2.0,-1.0,-7.0],
                                 vec![6.0,-1.0, 5.0]] };

        assert!( a.minor(0,0)    == -12.0 );
        assert!( a.cofactor(0,0) == -12.0 );
        assert!( a.minor(1,0)    ==  25.0 );
        assert!( a.cofactor(1,0) == -25.0 );
    }

    #[test]
    fn matrix_determinant_3_by_3(){
        let a = Matrix { cols: 3, rows: 3,
                         m: vec![vec![ 1.0, 2.0, 6.0],
                                 vec![-5.0, 8.0,-4.0],
                                 vec![ 2.0, 6.0, 4.0]] };

        assert!( a.cofactor(0,0) ==   56.0 );
        assert!( a.cofactor(0,1) ==   12.0 );
        assert!( a.cofactor(0,2) ==  -46.0 );
        assert_eq!( a.determinant(), -196.0 );
    }

    #[test]
    fn matrix_determinant_4_by_4(){
        let a = Matrix { cols: 4, rows: 4,
                         m: vec![vec![-2.0,-8.0, 3.0, 5.0],
                                 vec![-3.0, 1.0, 7.0, 3.0],
                                 vec![ 1.0, 2.0,-9.0, 6.0],
                                 vec![-6.0, 7.0, 7.0,-9.0]] };

        assert!( a.cofactor(0,0) ==   690.0 );
        assert!( a.cofactor(0,1) ==   447.0 );
        assert!( a.cofactor(0,2) ==   210.0 );
        assert!( a.cofactor(0,3) ==    51.0 );
        assert_eq!( a.determinant(), -4071.0 );
    }

    #[test]
    fn matrix_test_inversion_true(){
        let a = Matrix { cols: 4, rows: 4,
                         m: vec![vec![ 6.0, 4.0, 4.0, 4.0],
                                 vec![ 5.0, 5.0, 7.0, 6.0],
                                 vec![ 4.0,-9.0, 3.0,-7.0],
                                 vec![ 9.0, 1.0, 7.0,-6.0]] };

        assert_eq!( a.determinant(), -2120.0 );
        assert!( a.is_invertible() );
    }

    #[test]
    fn matrix_test_inversion_false(){
        let a = Matrix { cols: 4, rows: 4,
                         m: vec![vec![-4.0, 2.0,-2.0, 3.0],
                                 vec![ 9.0, 6.0, 2.0, 6.0],
                                 vec![ 0.0,-5.0, 1.0,-5.0],
                                 vec![ 0.0, 0.0, 0.0, 0.0]] };

        assert_eq!( a.determinant(), 0.0 );
        assert!( !a.is_invertible() );
    }

    #[test]
    fn matrix_inversion(){
        let a = Matrix { cols: 4, rows: 4,
                         m: vec![vec![-5.0, 2.0, 6.0,-8.0],
                                 vec![ 1.0,-5.0, 1.0, 8.0],
                                 vec![ 7.0, 7.0,-6.0,-7.0],
                                 vec![ 1.0,-3.0, 7.0, 4.0]] };
        let b = a.inverse();

        assert_eq!( a.determinant(), 532.0 );

        assert_eq!( a.cofactor(2,3), -160.0 );
        assert_eq!( b.get(3,2), -160.0/532.0 );

        assert_eq!( a.cofactor(3,2), 105.0 );
        assert_eq!( b.get(2,3), 105.0/532.0 );

        let expected = Matrix { cols: 4, rows: 4,
                         m: vec![vec![ 0.21805, 0.45113, 0.24060,-0.04511],
                                 vec![-0.80827,-1.45677,-0.44361, 0.52068],
                                 vec![-0.07895,-0.22368,-0.05263, 0.19737],
                                 vec![-0.52256,-0.81391,-0.30075, 0.30639]] };

        assert!( b.equals(expected) );

    }

    #[test]
    fn matrix_invert_another(){
        let a = Matrix { cols: 4, rows: 4,
                         m: vec![vec![ 8.0,-5.0, 9.0, 2.0],
                                 vec![ 7.0, 5.0, 6.0, 1.0],
                                 vec![-6.0, 0.0, 9.0, 6.0],
                                 vec![-3.0, 0.0,-9.0,-4.0]] };
        
        let expected = Matrix { cols: 4, rows: 4,
                         m: vec![vec![-0.15385,-0.15385,-0.28205,-0.53846],
                                 vec![-0.07692, 0.12308, 0.02564, 0.03077],
                                 vec![ 0.35897, 0.35897, 0.43590, 0.92308],
                                 vec![-0.69231,-0.69231,-0.76923,-1.92308]] };

        assert!( a.inverse().equals(expected) );
    }

    #[test]
    fn matrix_invert_yet_another(){
        let a = Matrix { cols: 4, rows: 4,
                         m: vec![vec![ 9.0, 3.0, 0.0, 9.0],
                                 vec![-5.0,-2.0,-6.0,-3.0],
                                 vec![-4.0, 9.0, 6.0, 4.0],
                                 vec![-7.0, 6.0, 6.0, 2.0]] };
        
        let expected = Matrix { cols: 4, rows: 4,
                         m: vec![vec![-0.04074,-0.07778, 0.14444,-0.22222],
                                 vec![-0.07778, 0.03333, 0.36667,-0.33333],
                                 vec![-0.02901,-0.14630,-0.10926, 0.12963],
                                 vec![ 0.17778, 0.06667,-0.26667, 0.33333]] };

        assert!( a.inverse().equals(expected) );
    }
}

