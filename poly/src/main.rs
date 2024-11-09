use poly::component::{leaf, composite};
use poly::materials::material;

fn main() {
    let l1 = leaf(Some(material()));
    let l2 = leaf(Some(material()));
    let mut c1 = composite();
    let mut c2 = composite();
    
    c1.add(l1);
    c2.add(c1);
    c2.add(l2);

    println!("{:#?}", c2);
}
