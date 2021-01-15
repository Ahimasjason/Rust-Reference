fn give_me<T>(value: T) {
    let _ = value;
}

fn main() {
    let a = "generics";
    let b = 1234;
    give_me(a);
    give_me(b);
}
