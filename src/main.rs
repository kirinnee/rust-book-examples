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
    loop {
        let s = ask_for_input("Input a phrase: ", |x| match x.len() {
            0 => Err("Please enter a string".to_string()),
            _ => Ok(x.to_string()),
        });

        let mut iter = s.chars();
        let y = iter.next();
        let mut left = iter.collect::<String>();
        if let Some(c) = y {
            let output = match c {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    let out = format!("{}{}-hay", c, left);
                    out
                }
                _ => {
                    left.push('-');
                    left.push(c);
                    left + "ay"
                }
            };
            println!("Pig latin: {}", output)
        }
    }
}



