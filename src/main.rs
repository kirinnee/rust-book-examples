use std::io::stdin;

fn ask_for_input<F, T>(question: &str, f: F) -> T where F: (Fn(&str) -> Result<T, String>) {
    loop {
        println!("{}", question);
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let out = input.trim();
        match f(out) {
            Ok(e) => break e,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }
    }
}

fn main() {
    let s1 = String::from("Hello ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2;

    let s4 = "Hello ".to_owned() + "World";

    println!("{}", s3);
    println!("{}", s4);

    let hello = "Здравствуйте";

    let s = &hello[0..2];

    println!("{}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}



