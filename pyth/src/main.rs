use std::i64;

fn main() {
    let x = no_semicolon();
    println!("here is the number {}", x);
    let y: i64 = plus_one(4);
    println!("here is the {}", y)
}

fn no_semicolon() -> i32 {
    9
}

fn plus_one(x: i64) -> i64 {
    return x + 1;
}
