use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let secret_num = rand::thread_rng().gen_range(1..=100);

    println!("guess the number !!");

    loop {
        let mut guess = String::new();
        println!("take your guess: ");
        io::stdin().read_line(&mut guess).expect("catch error");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("your guess is: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("your number is smaller "),
            Ordering::Greater => println!("your number is greater"),
            Ordering::Equal => {
                println!("your guess is correct");
                break;
            }
        }
    }
}
