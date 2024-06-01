
#[derive(Debug)]
pub struct Sphere {
}

impl Sphere {
    pub fn new() -> Self {
        Self {}
    }
    
}

#[cfg(test)]
mod tests {
    use crate::spheres::Sphere;

    #[test]
    fn new_creates_unique_spheres(){
        let s1 = Sphere::new();
        let s2 = Sphere::new();
        assert_ne!( &s1 as *const _, &s2 as *const _ ); 
    }

}
