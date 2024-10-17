use crate::component::Component;

pub struct Composite {
}

impl Component for Composite {
    fn operation(&self) -> usize{
        0
    }
}
