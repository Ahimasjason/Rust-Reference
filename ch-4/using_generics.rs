fn main() {
    //providing Type
    let v1: Vec<&str> = Vec::new();
    // Calling method
    let mut v2 = Vec::new();
    v2.push(2);
    // turbofish
    let v3 = Vec::<u8>::new();
    generic_fn();
}
use std::str;

fn generic_fn() {
    let num_from_str = str::parse::<u8>("34");
    println!("Number from Str {:?}", num_from_str);
}
