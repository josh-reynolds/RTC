
#[derive(Debug,Clone)]
pub struct Coordinate {
    pub index: usize,
}

pub fn coordinate() -> Coordinate {
    Coordinate {
        index: 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::coordinate::coordinate;

    #[test]
    fn constructing_a_coordinate(){
        let c = coordinate();
        assert!(c.index == 0);
    }
}


