use crate::lights::Light;
use crate::spheres::Sphere;

#[derive(Debug)]
pub struct World{
    pub light: Option<Light>,
    pub objects: Vec<Sphere>,   // only have spheres, need to think about 'Object' 
}                               // parent class and how to implement properly

pub fn world() -> World {
    World { 
        light: None,
        objects: vec![] }
}


#[cfg(test)]
mod tests {
    use crate::world::world;

    #[test]
    fn creating_a_world(){
        let w = world();
        assert!( match w.light {
                   Some(_) => false,
                   None => true,
        });
        assert!( w.objects.len() == 0 );
    }
}
