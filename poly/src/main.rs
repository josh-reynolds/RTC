use poly::component::{leaf, composite};

fn main() {
    let l = leaf();
    let mut c1 = composite();
    let mut c2 = composite();
    c1.add(l);
    c2.add(c1);

    println!("{:?}", c2);

    let ch = c2.get_child(0)
               .expect("VALID INDEX")
               .get_child(0)
               .unwrap();

    println!("{:?}", ch);
}
