use std::fmt::Debug;

fn add_things<T: Debug>(a: T, b: T) {
    // a + b
    println!("A : {:?} ", a);
}

fn where_display<T>(val: T)
where
    T: Debug,
{
    println!("Value is {:?}", val);
}

fn main() {
    add_things(2, 2);
    where_display(9);
}
