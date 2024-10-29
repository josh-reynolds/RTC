use crate::coordinate::{Coordinate,coordinate};
// TO_DO:
//   coordinate/index values
//   Visitor class for operations

#[derive(Debug,Clone)]
pub enum Component {
    Leaf { value: usize,
           index: Coordinate },
    Composite { value: usize,
                children: Vec<Component>,
                index: Coordinate },
}

impl Component {
    pub fn operation(self) -> usize {
        match self {
            Component::Leaf{value,index: _} => value,
            Component::Composite{value,children: _,index: _} => value,
        }
    }

    pub fn add(&mut self, c: Component) -> Option<Coordinate> {
        match self {
            Component::Leaf{value: _,index: _} => None,
            Component::Composite{value: _, children: ch,index: _} => { 
                ch.push(c);
                Some(coordinate(ch.len() - 1))
            },
        }
    }

    //pub fn remove(&mut self, c: Component);
    
    pub fn get_child(&self, i: usize) -> Option<Component> {
        match self {
            Component::Leaf{value: _, index: _} => None,
            Component::Composite{value: _, children: ch, index: _} => {
                if i >= ch.len() {
                    None      // not sure if we should assert here instead...
                } else {
                    Some(ch[i].clone())  
                }
            }
        }
    }
}

pub fn leaf() -> Component {
    Component::Leaf { 
        value: 1,
        index: coordinate(0),
    }
}

pub fn composite() -> Component {
    Component::Composite { 
        value: 0,
        children: vec!(),
        index: coordinate(0),
    }
}

#[cfg(test)]
mod tests {
    use crate::component::{Component, leaf, composite};

    #[test]
    fn constructing_a_leaf(){
        let l = leaf();
        assert!(l.operation() == 1);
    }
    

    #[test]
    fn constructing_a_composite(){
        let c = composite();
        assert!(c.operation() == 0);
    }

    #[test]
    fn adding_to_a_composite(){
        let mut c = composite();
        let l = leaf();
        let index = c.add(l).unwrap();

        assert!(index.index == 0);

        if let Component::Composite{value: _, children: ch, index: _} = c {
            assert!(ch.len() == 1);
        }
    }

    #[test]
    fn add_returns_coordinate(){
        let mut c = composite();
        let l = leaf();
        let index = c.add(l).unwrap();

        assert!(index.index == 0);
    }
}
