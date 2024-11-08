
#[derive(Debug,Clone)]
pub struct Material {
    pub value: f64,
}

pub fn material() -> Material {
    Material { 
        value: 4.3,
    }
}
