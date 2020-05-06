use std::io::stdin;
use std::num::ParseIntError;

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

fn largest<T: PartialOrd + Copy>(list: &[T]) -> Option<T> {
    let mut largest = *list.get(0)?;

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}

fn main() {
    let ask = |x: &str| -> Result<Vec<i64>, String>{
        x.split_ascii_whitespace().map(|x| x.parse().map_err(|x: ParseIntError| x.to_string())).collect()
    };

    loop {
        let numbers = ask_for_input("Enter a integer array: ", ask);

        match largest(&numbers) {
            Some(huge) => println!("{}", huge),
            None => println!("Nothing was found :<"),
        }
    }
}



