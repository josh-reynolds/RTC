use poly::component::{leaf, composite};
use poly::materials::material;

fn main() {
    let l = leaf(Some(material()));
    let mut c1 = composite();
    let mut c2 = composite();
    
    c1.add(l.clone());
    c2.add(c1);

    println!("{:#?}", c2);
}
