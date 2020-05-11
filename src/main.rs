use std::io::stdin;

#[macro_export]
macro_rules! vv {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[macro_export]
macro_rules! mm {
    ( $($k: expr => $v: expr), *) => {
        {
            let temp_map = std::collections::HashMap::new();
            $(
                temp_map.insert($k, $v)
            )*
            temp_map
        }
    }
}

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

fn main() {
    let x = vv![1,2,3,4];
    println!["Hello World!"];

    let x = mm!(1 => 2, 3=> 4,);
}



