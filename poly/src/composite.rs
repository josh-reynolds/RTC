//use crate::component::{IComponent,Component};

//pub struct Composite {
    //value: usize,
    //children: Vec<Component>
//}

//impl IComponent for Composite {
    //fn operation(&self) -> usize{
        //self.value
        //for all g in children: g.operation()
    //}

    //fn add(&mut self, c: Component) -> Option<usize> {
        //self.children.push(c);
        //Some(self.children.len() - 1)
    //}

    //fn remove(&self, Component: c);
    //fn get_child(&self, usize: i);
//}

//pub fn composite() -> Component {
    //Composite{ 
        //value: 0,
        //children: vec!(),
    //}
//}

//#[cfg(test)]
//mod tests {
    //use crate::composite::composite;
    //use crate::component::IComponent;
//
    //#[test]
    //fn constructing_a_composite(){
        //let c = composite();
        //assert!(c.operation() == 0);
    //}
//}
