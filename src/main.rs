use std::cmp::Ordering;
use std::collections::HashMap;
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

fn main() {
    let asker = |x: &str| -> Result<Vec<i64>, String> {
        x.split_ascii_whitespace()
            .map(|x|
                x.parse()
                    .map_err(|err: ParseIntError| err.to_string()))
            .collect()
    };

    loop {
        let mut v = ask_for_input("Enter an array for integers", asker);
        let sum: i64 = v.iter().sum();
        let avg = sum as f32 / v.len() as f32;

        v.sort();
        let median = v[v.len() / 2];

        let mut occurence = HashMap::new();

        for i in v.into_iter() {
            let count = occurence.entry(i).or_insert(0);
            *count += 1;
        }

        println!("Average: {}", avg);
        println!("Median: {}", median);

        if let Some((k, v)) = occurence.into_iter()
            .max_by(|(_, av), (_, bv)|
                if *av > *bv {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            ) {
            println!("Max occurrence: {} with {} times", k, v);
        } else {
            println!("no max");
        }
    }
}



