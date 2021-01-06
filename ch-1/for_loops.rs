fn main() {
    println!("Normal Range");

    for i in 0..10 {
        println!("{}", i);
    }

    println!();

    println!("Inclusive Range");

    // Count till 10

    for i in 0..=10 {
        println!("{}", i);
    }
}
