use crate::component::Component;

pub struct Leaf {
    value: usize,
}

impl Component for Leaf {
    fn operation(&self) -> usize{
        self.value
    }
    //fn add(&self, Component: c);
    //fn remove(&self, Component: c);
    //fn get_child(&self, usize: i);
}

pub fn leaf() -> Leaf {
    Leaf{ value: 1 }
}

#[cfg(test)]
mod tests {
    use crate::leaf::leaf;
    use crate::component::Component;

    #[test]
    fn constructing_a_leaf(){
        let l = leaf();
        assert!(l.operation() == 1);
    }
}
