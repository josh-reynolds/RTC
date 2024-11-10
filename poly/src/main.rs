use poly::component::{leaf, composite};
use poly::materials::material;

fn main() {
    let l1 = leaf(Some(material()));
    let l2 = leaf(Some(material()));

    let mut c1 = composite();
    let mut c2 = composite();
    
    let a = c1.add(l1);
    let b = c2.add(l2);
    let c = c2.add(c1);

    println!("{:#?}", c2);
    println!("{:?} {:?} {:?}", a, b, c);
}

//        c2
//      /    \
//     l2    c1
//           /
//          l1
