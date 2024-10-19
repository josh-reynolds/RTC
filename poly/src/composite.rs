use crate::component::Component;

pub struct Composite {
    value: usize,
    //children: Vec<Box<dyn Component>>
}

impl Component for Composite {
    fn operation(&self) -> usize{
        self.value
        //for all g in children: g.operation()
    }
    //fn add(&self, Component: c);
    //fn remove(&self, Component: c);
    //fn get_child(&self, usize: i);
}

pub fn composite() -> Composite {
    Composite{ value: 0 }
}

#[cfg(test)]
mod tests {
    use crate::composite::composite;
    use crate::component::Component;

    #[test]
    fn constructing_a_composite(){
        let c = composite();
        assert!(c.operation() == 0);
    }
}
