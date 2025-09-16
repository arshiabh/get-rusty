use std::i64;

fn main() {
    let x = no_semicolon();
    println!("here is the number {}", x);
    let y: i64 = plus_one(4);
    println!("here is the {}", y);
    //
    let z = looping();
    println!("the looping res {}", z);
    //
    loop_list();
}

fn loop_list() {
    let a = ["apple", "banana", "pineapple"];
    let mut x = 0;
    while x < a.len() {
        println!("printing every item in list {}", a[x]);
        x += 1;
    }

    for element in a {
        println!("with for loop {}", element)
    }

    for i in (1..4).rev() {
        println!("fancy loop {}", i)
    }
}

fn no_semicolon() -> i32 {
    let x: i32 = 22;
    x + 1
}

fn plus_one(x: i64) -> i64 {
    return x + 1;
}

fn looping() -> i64 {
    let mut number: i64 = 0;
    loop {
        number += 1;
        if number == 10 {
            return number * 2;
        }
    }
}
