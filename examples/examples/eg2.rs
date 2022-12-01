use searchsorted::{cartesian_5d,searchsorted_5d};

use rand::{thread_rng, Rng};
use rand::prelude::IteratorRandom;
use rand::distributions::Alphanumeric;
use std::time::SystemTime;

fn main() {
    let lst1 = (0..100).map(|_| rand_strings(2)).collect::<Vec<String>>();
    let lst2 = (0..10).map(|_| rand_strings(2)).collect::<Vec<String>>();
    let lst3 = (0..10).map(|_| rand_strings(2)).collect::<Vec<String>>();
    let lst4 = (0..10).map(|_| rand_strings(2)).collect::<Vec<String>>();
    let lst5 = (0..10).map(|_| rand_strings(2)).collect::<Vec<String>>();

    let start = SystemTime::now();
    let full_list = cartesian_5d(lst1,lst2,lst3,lst4,lst5);
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("Pure Rust eTime {} μs.   cartesian_5d", duration.as_micros());

    let mut rng = thread_rng();
    let sample_list = full_list.iter().cloned().choose_multiple(&mut rng, 1000);


    let start = SystemTime::now();
    searchsorted_5d(full_list, sample_list);
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("Pure Rust eTime {} μs.   searchsorted_5d", duration.as_micros());

}

fn rand_strings(n:usize) -> String {  
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .map(char::from) // map added here
        .take(n)
        .collect();
    rand_string
}

// let part = Vec::from_iter(data[1..4].iter().cloned());