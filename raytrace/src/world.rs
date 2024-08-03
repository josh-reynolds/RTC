use crate::lights::{Light, point_light};
use crate::spheres::sphere;
use crate::shapes::Shape;
use crate::tuple::{Tuple, point};
use crate::color::{Color, color};
use crate::materials::{material, lighting};
use crate::transform::scaling;
use crate::rays::{Ray, ray};
use crate::intersections::{Intersection, hit, Computations, prepare_computations};

#[derive(Debug)]
pub struct World {
    pub light: Option<Light>,
    objects: Vec<Box<dyn Shape>>,
}

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

        let shadowed = self.is_shadowed(comps.over_point);

        lighting(self.objects[comps.object].get_material().clone(), &l, comps.point, comps.eyev, comps.normalv, shadowed)
    }

    pub fn color_at(&self, r: Ray) -> Color {
        let xs = self.intersect(r);
        let n = xs.len();
        if n == 0 || xs[n-1].t < 0.0 {
            return color(0.0, 0.0, 0.0)
        } else {
            let mut hit = xs[0];
            if hit.t < 0.0 {
                for i in xs {
                    if i.t >= 0.0 {
                        hit = i;
                        break;
                    }
                }
            }
            let comps = prepare_computations(hit, r, self);
            self.shade_hit(comps)
        }
    }

    pub fn is_shadowed(&self, p: Tuple) -> bool {
        let v = self.light.as_ref().unwrap().position - p;
        let distance = v.mag();
        let direction = v.normal();

        let r = ray(p, direction);
        let xs = self.intersect(r);

        let mut result = false;
        if xs.len() > 0 {
            let h = hit(xs);
            let t = match h {
                Some(hit) => hit.t,
                None => distance + 1.0
            };
            if t < distance {
                result = true;
            }
        }

        result
    }

    pub fn add(&mut self, mut obj: Box<dyn Shape>){
        let current = self.objects.len();
        obj.set_index( current as usize );
        self.objects.push( obj );
    }

    pub fn get_object(&self, index: usize) -> &Box<dyn Shape> {
        &(self.objects[index])
    }
}

pub fn world() -> World {
    World { 
        light: None,
        objects: vec![],
    }
}

pub fn default_world() -> World {
    let mut s1 = sphere();
    let mut m = material();
    m.color = color(0.8, 1.0, 0.6);
    m.diffuse = 0.7;
    m.specular = 0.2;
    s1.set_material( m );

    let mut s2 = sphere();
    let t = scaling(0.5, 0.5, 0.5);
    s2.set_transform( t );

    World { 
        light: Some( point_light(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0))),
        objects: vec![Box::new(s1), Box::new(s2)],
    }
}

#[cfg(test)]
mod tests {
    use crate::equals::EPSILON;
    use crate::world::{world, default_world};
    use crate::color::color;
    use crate::tuple::{point, vector};
    use crate::spheres::sphere;
    use crate::shapes::Shape;
    use crate::transform::{scaling, translation};
    use crate::lights::point_light;
    use crate::materials::material;
    use crate::rays::ray;
    use crate::intersections::{intersection, prepare_computations};
    use crate::planes::plane;
    use crate::matrix::identity;

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
        s1.set_material( m );

        let mut s2 = sphere();
        let t = scaling(0.5, 0.5, 0.5);
        s2.set_transform( t );

        let w = default_world();

        assert!( match w.light {
                   Some(lgt) => lgt.equals( l ),
                   None => false, 
        });
        assert!( w.objects.contains( &(Box::new(s1) as Box<dyn Shape>) ));
        assert!( w.objects.contains( &(Box::new(s2) as Box<dyn Shape>) ));
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
        let i = intersection(4.0, 0);
        let comps = prepare_computations(i, r, &w);

        let c = w.shade_hit(comps);
        assert!( c.equals( color(0.38066, 0.47583, 0.2855) ));
    }

    #[test]
    fn shading_an_intersection_from_inside(){
        let mut w = default_world();
        w.light = Some(point_light( point(0.0, 0.25, 0.0), color(1.0, 1.0, 1.0) ));
        let r = ray( point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0) );
        let i = intersection(0.5, 1);
        let comps = prepare_computations(i, r, &w);

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

        let mut mat = material();
        mat.ambient = 1.0;
        w.objects[0].set_material( mat );

        let mut mat = material();
        mat.ambient = 1.0;
        w.objects[1].set_material( mat );

        let r = ray( point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0) );

        let c = w.color_at(r);
        assert!( c.equals( w.objects[1].get_material().color ));
    }

    #[test]
    fn color_with_all_intersections_behind_ray(){
        let w = default_world();
        let r = ray( point(0.0, 0.0, 10.0), vector(0.0, 0.0, 1.0) );

        let c = w.color_at(r);
        assert!( c.equals( color(0.0, 0.0, 0.0) ));
    }

    #[test]
    fn shading_when_nothing_colinear(){
        let w = default_world();
        let p = point(0.0, 10.0, 0.0);

        assert!( !w.is_shadowed(p) );
    }

    #[test]
    fn shading_when_object_between_point_and_light(){
        let w = default_world();
        let p = point(10.0, -10.0, 10.0);

        assert!( w.is_shadowed(p) );
    }

    #[test]
    fn shading_when_light_between_point_and_object(){
        let w = default_world();
        let p = point(-20.0, 20.0, -20.0);

        assert!( !w.is_shadowed(p) );
    }

    #[test]
    fn shading_when_point_between_light_and_object(){
        let w = default_world();
        let p = point(-2.0, 2.0, -2.0);

        assert!( !w.is_shadowed(p) );
    }

    #[test]
    fn shade_hit_given_an_intersection_in_shadow(){
        let mut w = world();
        w.light = Some(point_light( point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0) ));
        let s1 = sphere();
        let mut s2 = sphere();
        s2.set_transform( translation(0.0, 0.0, 10.0) );
        w.add(Box::new(s1));
        w.add(Box::new(s2));
        
        let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let i = intersection(9.0, 1);   // I think the book has a typo here
        let comps = prepare_computations(i, r, &w);
        
        let c = w.shade_hit(comps);

        assert!( c.equals(color(0.1, 0.1, 0.1)));
    }

    #[test]
    fn hit_should_offset_point(){
        let mut w = world();
        let mut s = sphere();
        s.set_transform( translation(0.0, 0.0, 1.0) );
        w.add(Box::new(s));

        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let i = intersection(5.0, 0);
        let comps = prepare_computations(i, r, &w);
    
        assert!( comps.over_point.z < -EPSILON/2.0 );
        assert!( comps.point.z > comps.over_point.z );
    }

    #[test]
    fn can_add_plane_to_objects(){
        let mut w = world();
        let p = plane();
        w.add( Box::new(p) );

        assert!( w.objects.len() == 1 );

        let t = &w.objects[0];  
        let trans = t.get_transform();
        let mat = t.get_material();

        assert!( *trans == identity() );
        assert!( *mat == material() );
    }

    #[test]
    fn adding_object_sets_index(){
        let mut w = world();

        let mut s1 = sphere();
        let mut m1 = material();
        m1.color = color(1.0, 0.0, 0.0);
        s1.set_material(m1);
        w.add(Box::new(s1));

        let mut p = plane();
        let mut m2 = material();
        m2.color = color(0.0, 1.0, 0.0);
        p.set_material(m2);
        w.add(Box::new(p));
        
        let mut s2 = sphere();
        let mut m3 = material();
        m3.color = color(0.0, 0.0, 1.0);
        s2.set_material(m3);
        w.add(Box::new(s2));

        assert!( w.objects.len() == 3 );
        
        assert!( w.get_object(0).get_index() == 0 );
        assert!( w.get_object(0).get_material().color.equals( color(1.0, 0.0, 0.0) ));

        assert!( w.get_object(1).get_index() == 1 );
        assert!( w.get_object(1).get_material().color.equals( color(0.0, 1.0, 0.0) ));
                 
        assert!( w.get_object(2).get_index() == 2 );
        assert!( w.get_object(2).get_material().color.equals( color(0.0, 0.0, 1.0) ));
    }
}
