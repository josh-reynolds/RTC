use crate::shapes::{Base, shape};

pub struct Cylinder {
    supe: Base,
}

pub fn cylinder() -> Cylinder {
    Cylinder {
        supe: shape(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cylinders::cylinder;

    #[test]
    fn a_ray_misses_a_cylinder(){
        let cyl = cylinder();
    }
}

