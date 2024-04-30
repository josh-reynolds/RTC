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

    pub fn equals(&self, m: Matrix) -> bool {
        ( self.rows == m.rows ) &&
        ( self.cols == m.cols ) &&
        equals(&self.m, &m.m )
    }
}

// very similar function in Canvas, may want to refactor all
// this later
pub fn equals(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> bool {
    if (a.len() != b.len()) || (a[0].len() != b[0].len()){
        return false
    }

    for n in 0..(a.len()) {
        for m in 0..(a[n].len()){
            if a[n][m] != b[n][m]{
                return false
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

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
}
