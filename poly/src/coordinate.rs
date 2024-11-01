
#[derive(Debug,Clone)]
pub struct Coordinate {
    pub index: usize,
    pub parent: usize,
}

pub fn coordinate(i: usize) -> Coordinate {
    Coordinate {
        index: i,
        parent: 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::coordinate::coordinate;

    #[test]
    fn constructing_a_coordinate(){
        let c = coordinate(0);
        assert!(c.index == 0);
        assert!(c.parent == 0);
    }
}


