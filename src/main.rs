use std::io::stdin;

fn ask_for_input<F>(question: &str, f: F) -> String where F: Fn(&str) -> bool {
    loop {
        println!("{}", question);
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let out = input.trim();
        if f(out) {
            break out.to_string();
        } else {
            continue;
        }
    }
}

fn main() {
    println!("Hello World!");
}



