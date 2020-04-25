use std::collections::HashMap;
use std::io::stdin;

fn main() {
    loop {
        println!("Please enter the x-th fibonacci sequence");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Cannot read line");

        let n: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut cache = HashMap::new();
        cache.insert(1, 1);
        let fib = fib(n, &mut cache);
        println!("Value: {}", fib);
    }
}

fn fib(x: i32, cache: &mut HashMap<i32, u64>) -> u64 {
    match cache.get(&x) {
        Some(number) => *number,
        _ if x > 1 => {
            let val = fib(x - 1, cache) + fib(x - 2, cache);
            cache.insert(x, val);
            val
        }
        _ => x as u64,
    }
}


