//use crate::composite::Composite;
//use crate::leaf::Leaf;

pub enum Comp {
    Leaf,
    Composite,
}

pub trait Component {
    fn operation(&self) -> usize;
    fn add(&mut self, c: Comp) -> Option<usize>;
    //fn remove(&self, c: Component);
    //fn get_child(&self, i: usize);
}
