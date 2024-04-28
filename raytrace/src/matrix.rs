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
}

