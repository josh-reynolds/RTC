use poly::component::{leaf, composite};

fn main() {
    let l = leaf();
    let mut c1 = composite();
    let mut c2 = composite();
    
    //let _first = c1.add(l).unwrap();
    //let _second = c2.add(c1).unwrap();
    
    c1.add(l);
    c2.add(c1);

    println!("{:#?}", c2);

    //let grp = c2.get_child(first.index).unwrap();
    //println!("{:?}", grp);

    //let ch = c2.get_child(first.index)
               //.expect("VALID INDEX")
               //.get_child(second.index)
               //.unwrap();

    //println!("{:?}", ch);

    //let r = &ch;
    //println!("{:?}", *r);
}
