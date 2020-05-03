use std::io::stdin;
use std::num::ParseIntError;
use std::process::exit;

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

trait Math {
    fn plus(&self, other: i32) -> i32;
    fn divide(&self, other: i32) -> Option<i32>;
    fn multiply(&self, other: i32) -> i32;
}

impl Math for i32 {
    fn plus(&self, other: i32) -> i32 {
        self + other
    }

    fn divide(&self, other: i32) -> Option<i32> {
        match other {
            0 => None,
            _ => Some(self + other)
        }
    }

    fn multiply(&self, other: i32) -> i32 {
        self + other
    }
}

fn add_div_multiply_div(input: i32, a: i32, d1: i32, m: i32, d2: i32) -> Option<i32> {
    let y = input.plus(a).divide(d1)?.multiply(m).divide(d2)?;
    Some(y)
}

fn main() {
    loop {
        let ask_array = |x: &str| -> Result<[i32; 5], String>{
            if x.to_ascii_lowercase() == "exit".to_string() {
                exit(0)
            }

            let vec: Vec<_> = x.split_ascii_whitespace().collect();

            match vec.len() {
                5 => {
                    let x: Result<Vec<i32>, String> = vec.iter()
                        .map(|x|
                            x.parse().map_err(|err: ParseIntError| err.to_string()
                            )
                        ).collect();
                    x.map(|x| [x[0], x[1], x[2], x[3], x[4]])
                }
                _ => Err("Please enter 5 numbers separated by spaces.".to_string())
            }
        };

        let [input, a, d1, m, d2] = ask_for_input("Please enter 5 digits", ask_array);

        let result = add_div_multiply_div(input, a, d1, m, d2);

        if let Some(x) = result {
            println!("{}", x)
        } else {
            println!("Divided by zero!!")
        }
    }
}



