use crate::lights::{Light, point_light};
use crate::spheres::{Sphere,sphere};
use crate::tuple::point;
use crate::color::{Color, color};
use crate::materials::{material, lighting};
use crate::transform::scaling;
use crate::rays::Ray;
use crate::intersections::{Intersection, Computations, prepare_computations};

#[derive(Debug)]
pub struct World{
    pub light: Option<Light>,
    pub objects: Vec<Sphere>,   // only have spheres, need to think about 'Object' 
}                               // parent class and how to implement properly

impl World {
    pub fn intersect(&self, r: Ray) -> Vec<Intersection> {
        let mut result = vec!();

        for obj in &self.objects {
            let mut xs = obj.intersect(r);
            if xs.len() > 0 {
                result.append(&mut xs);
            }
        }

        result.sort_by( |a, b| a.t.partial_cmp(&b.t).unwrap() );
        result
    }

    pub fn shade_hit(&self, comps: Computations) -> Color {
        let binding = point_light( point(0.0, 0.0, 0.0), color(0.0, 0.0, 0.0) );
        let l = match &self.light {
            Some(lgt) => lgt,
            None      => &binding,
        };

        lighting( comps.object.material, &l, comps.point, comps.eyev, comps.normalv )
    }

    pub fn color_at(&self, r: Ray) -> Color {
        let xs = self.intersect(r);
        if xs.len() == 0 {
            return color(0.0, 0.0, 0.0)
        } else {
            let mut hit = xs[0];
            if hit.t < 0.0 {
                for i in xs {
                    if i.t >= 0.0 {
                        hit = i;
                        break;
                    }
                    // need to consider case when all hits are behind (i.e. negative)
                }
            }
            let comps = prepare_computations(hit, r);
            self.shade_hit(comps)
        }
    }
}

pub fn world() -> World {
    World { 
        light: None,
        objects: vec![] }
}

pub fn default_world() -> World {
    let mut s1 = sphere();
    let mut m = material();
    m.color = color(0.8, 1.0, 0.6);
    m.diffuse = 0.7;
    m.specular = 0.2;
    s1.material = m;

    let mut s2 = sphere();
    let t = scaling(0.5, 0.5, 0.5);
    s2.set_transform( t );

    World { 
        light: Some( point_light(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0))),
        objects: vec![s1,s2] }
}

#[cfg(test)]
mod tests {
    use crate::world::{world, default_world};
    use crate::color::color;
    use crate::tuple::{point, vector};
    use crate::spheres::sphere;
    use crate::transform::scaling;
    use crate::lights::point_light;
    use crate::materials::material;
    use crate::rays::ray;
    use crate::intersections::{Intersection, prepare_computations};

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
        assert!( w.objects.contains( &s1 ));
        assert!( w.objects.contains( &s2 ));
    }

    #[test]
    fn intersect_world_with_ray(){
        let w = default_world();
        let r = ray( point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0) );

        let xs = w.intersect( r );

        assert_eq!( xs.len(), 4 );
        assert_eq!( xs[0].t, 4.0 );
        assert_eq!( xs[1].t, 4.5 );
        assert_eq!( xs[2].t, 5.5 );
        assert_eq!( xs[3].t, 6.0 );
    }

    #[test]
    fn shading_an_intersection(){
        let w = default_world();
        let r = ray( point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0) );
        let s = &w.objects[0];
        let i = Intersection::new(4.0, &s);
        let comps = prepare_computations(i, r);

        let c = w.shade_hit(comps);
        assert!( c.equals( color(0.38066, 0.47583, 0.2855) ));
    }

    #[test]
    fn shading_an_intersection_from_inside(){
        let mut w = default_world();
        w.light = Some(point_light( point(0.0, 0.25, 0.0), color(1.0, 1.0, 1.0) ));
        let r = ray( point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0) );
        let s = &w.objects[1];
        let i = Intersection::new(0.5, &s);
        let comps = prepare_computations(i, r);

        let c = w.shade_hit(comps);
        assert!( c.equals( color(0.90498, 0.90498, 0.90498) ));
    }

    #[test]
    fn color_when_ray_misses(){
        let w = default_world();
        let r = ray( point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0) );

        let c = w.color_at(r);
        assert!( c.equals( color(0.0, 0.0, 0.0) ));
    }

    #[test]
    fn color_when_ray_hits(){
        let w = default_world();
        let r = ray( point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0) );

        let c = w.color_at(r);
        assert!( c.equals( color(0.38066, 0.47583, 0.2855) ));
    }

    #[test]
    fn color_with_intersection_behind_ray(){
        let mut w = default_world();
        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;
        let r = ray( point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0) );

        let c = w.color_at(r);
        println!("{:?}", c);
        assert!( c.equals( w.objects[1].material.color ));

    }
}
