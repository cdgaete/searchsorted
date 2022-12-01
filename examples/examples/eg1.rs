use searchsorted::{cartesian_3d,searchsorted_3d};

fn main() {
    let sample = vec![(String::from("a1"),String::from("b2"),String::from("c1")),(String::from("a2"),String::from("b3"),String::from("c2"))];

    let dim1 = vec![String::from("a1"),String::from("a2")];
    let dim2 = vec![String::from("b1"),String::from("b2"),String::from("b3")];
    let dim3 = vec![String::from("c1"),String::from("c2")];

    let full = cartesian_3d(dim1,dim2,dim3);

    let index = searchsorted_3d(full, sample);

    println!("Location of sample tuples in full: {:?}", index)
}