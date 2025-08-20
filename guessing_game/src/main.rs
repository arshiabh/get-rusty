use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let mut guess = String::new();
    let secret_num = rand::thread_rng().gen_range(1..=100);

    println!("here is the guessing_game !!");
    println!("secret number is {}", secret_num);

    println!("take your guess");
    io::stdin().read_line(&mut guess).expect("catch error");

    let guess: u32 = guess.trim().parse().expect("expected number");

    println!("your guessed number is {}", guess);

    match guess.cmp(&secret_num) {
        Ordering::Less => println!("your number is smaller "),
        Ordering::Equal => println!("your guess is correct"),
        Ordering::Greater => println!("your number is greater"),
    }
}
