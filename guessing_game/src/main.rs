use std::io;

fn main() {
    let mut x = String::new();
    println!("here is the guessing_game lol!!");

    io::stdin().read_line(&mut x).expect("catch error");

    println!("your number is {} :))", x)
}
