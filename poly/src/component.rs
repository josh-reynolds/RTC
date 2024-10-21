#[derive(Debug)]
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
