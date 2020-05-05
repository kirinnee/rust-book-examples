use std::collections::HashMap;
use std::io::stdin;

use crate::Operations::{Add, List, Remove};

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

enum Operations {
    Add(String, String),
    List(String),
    Remove(String, String),
}

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();


    let ask = |x: &str| -> Result<Operations, String> {
        let v = x.split_ascii_whitespace().collect::<Vec<_>>();
        match *v {
            ["Add", human, "to", department] => Ok(Add(human.to_string(), department.to_string())),
            ["Remove", human, "from", department] => Ok(Remove(human.to_string(), department.to_string())),
            ["List", department] => Ok(List(department.to_string())),
            _ => Err(String::from("Unknown command")),
        }
    };

    let mut command = String::from("HR Interface ready");

    loop {
        let terminal = String::from("=> ") + &command;
        let operation = ask_for_input(&terminal, ask);
        command = match operation {
            Add(human, department) => {
                let entry = company.entry(department).or_insert(vec![]);
                entry.push(human);
                entry.sort();
                String::from("added")
            }
            List(department) => {
                let entry = company.entry(department).or_insert(vec![]);
                entry.sort();
                entry.join(", ")
            }
            Remove(human, department) => {
                let entry = company.entry(department).or_insert(vec![]);
                match entry.binary_search(&human) {
                    Ok(index) => {
                        entry.remove(index);
                        String::from("removed")
                    },
                    Err(_) => {
                        String::from("cannot find person")
                    }
                }
            }
        }
    }
}



