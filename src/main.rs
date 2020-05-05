use std::collections::HashMap;
use std::io::stdin;

use crate::Operations::*;

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
    Put(String, String),
    Get(String),
    Delete(String),
    List,
}

fn main() {
    let mut in_mem_db = HashMap::<String, String>::new();

    let asker = |x: &str| -> Result<Operations, String> {
        let v: Vec<_> = x.split_ascii_whitespace().collect();

        match *v.as_slice() {
            ["get", s] => Ok(Get(s.to_string())),
            ["del", s] => Ok(Delete(s.to_string())),
            ["put", k, v] => Ok(Put(k.to_string(), v.to_string())),
            ["list"] => Ok(List),
            _ => Err("Unknown Command".to_string()),
        }
    };


    let mut response = String::from("db started");

    loop {
        let operation = ask_for_input(&("=> ".to_owned() + &response), asker);
        match operation {
            Delete(k) => match in_mem_db.remove(&k) {
                None => { response = "Key not found".to_string() }
                Some(k) => { response = k + " is deleted" }
            }
            Put(k, v) => {
                in_mem_db.insert(k, v);
            }
            Get(k) => match in_mem_db.get(&k) {
                Some(v) => {
                    response = String::from("Value: ") + v
                }
                None => {
                    response = String::from("Key not found")
                }
            },
            List => {
                let vec: Vec<String> = in_mem_db.keys().map(|x| x.to_string()).collect();
                response = vec.join("\n");
            }
        }
    }
}



