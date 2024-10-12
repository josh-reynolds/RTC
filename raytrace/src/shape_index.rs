
#[derive(Clone,PartialEq,Debug,Copy)]
pub struct ShapeIndex {
    pub index: usize,
    pub parent: Option<usize>,
    pub grandparent: Option<usize>,
    pub greatparent: Option<usize>,  // arbitrary depth limit of four
}

impl ShapeIndex {
    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn set_index(&mut self, index: usize){
        self.index = index;
    }

    pub fn get_parent(&self) -> Option<usize>{
        self.parent
    }

    pub fn set_parent(&mut self, parent: usize){
        self.parent = Some(parent);
    }
}

pub fn shape_index() -> ShapeIndex {
    ShapeIndex{ 
        index: 0,
        parent: None,
        grandparent: None,
        greatparent: None, 
    }
}
// sketching out schema here:
//   accumulate a vector of indices as the 'coordinate' for a Shape
//   the right-most value is a non-Group Shape
//   the left-most value is found in World
//   
//   each intervening value is found in the Group referenced to its left
//
//   so complete lookup involves walking through chain of get_object() calls
//   (note: should unify method names, World::get_object() and Group::get_child())
//
//
// reconsidering... the simplest/dumbest approach is to have distinct parent
// fields. This would impose a depth limit - for now I think that's OK. If we also
// wrap the interactivity in method calls, we can hide the implementation and 
// change later if this limit becomes a problem.
