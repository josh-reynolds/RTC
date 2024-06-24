use crate::lights::{Light, point_light};
use crate::spheres::Sphere;
use crate::tuple::point;
use crate::color::color;

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

pub fn default_world() -> World {
    World { 
        light: Some( point_light(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0))),
        objects: vec![] }
}

#[cfg(test)]
mod tests {
    use crate::world::{world, default_world};
    use crate::color::color;
    use crate::tuple::point;
    use crate::spheres::sphere;
    use crate::transform::scaling;
    use crate::lights::point_light;
    use crate::materials::material;

    #[test]
    fn creating_a_world(){
        let w = world();
        assert!( match w.light {
                   Some(_) => false,
                   None => true,
        });
        assert!( w.objects.len() == 0 );
    }

    #[test]
    fn default_world_attributes(){
        let l = point_light( point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0) );

        let mut s1 = sphere();
        let mut m = material();
        m.color = color(0.8, 1.0, 0.6);
        m.diffuse = 0.7;
        m.specular = 0.2;
        s1.material = m;

        let mut s2 = sphere();
        let t = scaling(0.5, 0.5, 0.5);
        s2.set_transform( t );

        let w = default_world();
        assert!( match w.light {
                   Some(lgt) => lgt.equals( l ),
                   None => false, 
        });

    }
}
