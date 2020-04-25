use std::io::stdin;
use std::process::exit;

const KELVIN: f32 = 273.15;

fn ask() {
    println!("\tF - Fahrenheit");
    println!("\tC - Celsius");
    println!("\tK - Kelvin");
    println!("\tExit - Quit the program");
}

fn get_lowered() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Cannot read line");
    input.make_ascii_lowercase();
    input.trim().to_string()
}

fn obtain_type(question: &str) -> String {
    loop {
        println!("{}", question);
        ask();
        let from = get_lowered();

        match from.as_str() {
            "f" | "c" | "k" => break from,
            "exit" => exit(0),
            _ => continue,
        }
    }
}

fn ask_float(question: &str) -> f32 {
    loop {
        println!("{}", question);
        let from = get_lowered();

        match from.as_str() {
            "exit" => exit(0),
            any => break match any.parse() {
                Ok(e) => e,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            },
        }
    }
}

fn main() {
    loop {
        let from = obtain_type("Convert from which temperature scale: ");
        let to = obtain_type("Convert to which temperature scale: ");

        let f: Box<dyn Fn(f32) -> f32> = match (from.as_str(), to.as_str()) {
            ("f", "c") => Box::new(|x: f32| { (x - 32.0) * 5.0 / 9.0 }),
            ("c", "f") => Box::new(|x: f32| { x * 9.0 / 5.0 + 32.0 }),

            ("f", "k") => Box::new(|x: f32| { (x - 32.0) * 5.0 / 9.0 + KELVIN }),
            ("k", "f") => Box::new(|x: f32| { (x - KELVIN) * 9.0 / 5.0 + 32.0 }),

            ("k", "c") => Box::new(|x: f32| { x - KELVIN }),
            ("c", "k") => Box::new(|x: f32| { x + KELVIN }),
            _ if from == to => Box::new(|x: f32| { x }),
            _ => exit(0),
        };

        let value = ask_float("Please enter the value: ");
        let result = f(value);
        println!("{} {}", result, to.to_uppercase());
    }
}