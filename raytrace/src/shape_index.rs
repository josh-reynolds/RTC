
#[derive(Clone,PartialEq,Debug)]
pub struct ShapeIndex {
    pub parents: Vec<usize>,
}

pub fn shape_index() -> ShapeIndex {
    ShapeIndex{ parents: vec!() }
}
// sketching out schema here:
//   accumulate a vector of indices as the 'coordinate' for a Shape
//   the right-most value is a non-Group Shape
//   the left-most value is found in World
//   
//   each intervening value is found in the Group referenced to its left
//
//   so complete lookup involves walking through chain of get_object() calls
//   (note: should unify method names, World::get_object() and Group::get_shape())
//
