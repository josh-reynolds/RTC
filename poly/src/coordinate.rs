
#[derive(Debug,Clone)]
pub struct Coordinate {
    pub index: usize,
}

pub fn coordinate(i: usize) -> Coordinate {
    Coordinate {
        index: i,
    }
}

#[cfg(test)]
mod tests {
    use crate::coordinate::coordinate;

    #[test]
    fn constructing_a_coordinate(){
        let c = coordinate(0);
        assert!(c.index == 0);
    }
}


