fn increase_by(mut val: u16, how_much: u16) {
    val += how_much;
    println!("Value increased to {}", val);
}

fn main() {
    let score = 2000;
    increase_by(score, 300);
    let question = "How do you do ? ";

    println!("Score {} ques {}", score, question);
    println!("Another score {} ques {}", score, question);
}
