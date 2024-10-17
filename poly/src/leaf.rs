use crate::component::Component;

pub struct Leaf {
}

impl Component for Leaf {
    fn operation(&self) -> usize{
        0
    }
}
