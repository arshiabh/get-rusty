fn main() {
    let x: String = String::from("salam");
    let mut y = test(x);
    println!("check it out {}", y);
    //
    let some = check_len(&y);
    println!("here is the len {}", some);
    change_ref(&mut y);
    println!("y here is changed {}", y)
}

fn borrow() {
    let mut x = String::from("hello");
    {
        let y = &mut x;
        // drop after this scope
    }
    let mut z = &mut x;
}

fn change_ref(s: &mut String) {
    s.push_str(" added here");
}

fn check_len(s: &String) -> usize {
    s.len()
}

fn test(s: String) -> String {
    let l = s.len();
    println!("len of s is {}", l);
    return s;
}
