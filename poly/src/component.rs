use crate::coordinate::{Coordinate, coordinate};
use crate::materials::{Material, material};
// TO_DO:
//   coordinate/index values
//   Visitor class for operations

#[derive(Debug,Clone)]
pub enum Component {
    Leaf { value: usize,
           index: Coordinate,
           material: Material },
    Composite { value: usize,
                children: Vec<Component>,
                index: Coordinate },
}

impl Component {
    pub fn operation(self) -> usize {
        match self {
            Component::Leaf{value, index: _, material: _} => value,
            Component::Composite{value, children: _, index: _} => value,
        }
    }

    pub fn add(&mut self, c: Component) -> Option<Coordinate> {
        match self {
            Component::Leaf{value: _,index: _, material: _} => None,
            Component::Composite{value: _, children: ch, index: _} => { 
                ch.push(c);
                Some(coordinate(ch.len() - 1))
            },
        }
    }

    //pub fn remove(&mut self, c: Component);
    
    pub fn get_child(&self, i: usize) -> Option<Component> {
        match self {
            Component::Leaf{value: _, index: _, material: _} => None,
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
        material: material(),
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

    #[test]
    fn add_sets_index_correctly(){
        let mut c = composite();

        let l1 = leaf();
        let index1 = c.add(l1).unwrap();

        let l2 = leaf();
        let index2 = c.add(l2).unwrap();

        let l3 = leaf();
        let index3 = c.add(l3).unwrap();

        assert!(index1.index == 0);
        assert!(index2.index == 1);
        assert!(index3.index == 2);
    }

    #[test]
    fn add_sets_parent_correctly(){
        let mut c1 = composite();
        let c2 = composite();
        let mut c3 = composite();

        let l = leaf();
        let index = c3.add(l).unwrap();
        
        c1.add(c2);
        c1.add(c3);

        assert!(index.index == 0);
        //assert!(index.parent == 1);
    }
}
