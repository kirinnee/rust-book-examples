use std::fmt::{Display, Error, Formatter};
use std::io::stdin;

#[allow(dead_code)]
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


struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.0.join("_"))
    }
}

fn main() {
    let ask = |x: &str| -> Result<Vec<String>, String> {
        Ok(x.split_ascii_whitespace().map(|y| y.to_string()).collect())
    };
    loop {
        let out = ask_for_input("Enter sentence:", ask);
        println!("{}", Wrapper(out))
    }
}



