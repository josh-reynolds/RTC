// TO_DO:
//   coordinate/index values
//   Visitor class for operations

#[derive(Debug,Clone)]
pub enum Component {
    Leaf { value: usize },
    Composite { value: usize,
                children: Vec<Component>},
}

impl Component {
    pub fn operation(self) -> usize {
        match self {
            Component::Leaf{value} => value,
            Component::Composite{value,children: _} => value,
        }
    }

    pub fn add(&mut self, c: Component) -> Option<usize> {
        match self {
            Component::Leaf{value: _} => None,
            Component::Composite{value: _, children: ch} => { 
                ch.push(c);
                Some(ch.len() - 1)
            },
        }
    }

    //pub fn remove(&mut self, c: Component);
    
    pub fn get_child(&self, i: usize) -> Option<Component> {
        match self {
            Component::Leaf{value: _} => None,
            Component::Composite{value: _, children: ch} => {
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
    Component::Leaf { value: 1 }
}

pub fn composite() -> Component {
    Component::Composite { 
        value: 0,
        children: vec!(),
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
        let index = c.add(l);

        assert!(index == Some(0));

        if let Component::Composite{value: _, children: ch} = c {
            assert!(ch.len() == 1);
        }
    }
}
