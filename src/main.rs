use std::io::stdin;
use std::process::exit;

use crate::StackResponse::{Accepted, Error, OK, UnderFlow};
use crate::Token::*;

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

enum Token {
    Value(i64),
    Multiply,
    Divide,
    Add,
    Minus,
    Modulo,
    Pop,
    Swap,
    Stack,
    Negate,
    Absolute,
    Max,
    Min,
    Duplicate,
    Drop,
    Nip,
}

enum StackResponse {
    OK(String),
    Accepted,
    Error(String),
    UnderFlow,
}


trait SpecialStack {
    fn remove_and_add(&mut self, remove: usize, a: &mut Vec<i64>) -> StackResponse;
    fn last2(&self) -> Option<(i64, i64)>;
    fn mut1(&mut self, f: fn(i64) -> Vec<i64>) -> StackResponse;
    fn mut2(&mut self, f: fn(i64, i64) -> Vec<i64>) -> StackResponse;
    fn mut1_err(&mut self, f: fn(i64) -> Result<Vec<i64>, String>) -> StackResponse;
    fn mut2_err(&mut self, f: fn(i64, i64) -> Result<Vec<i64>, String>) -> StackResponse;
}

impl SpecialStack for Vec<i64> {
    fn remove_and_add(&mut self, remove: usize, a: &mut Vec<i64>) -> StackResponse {
        let mut left = remove;
        while left > 0 {
            if let None = self.pop() {
                return UnderFlow;
            }
            left -= 1;
        }

        self.append(a);
        Accepted
    }

    fn last2(&self) -> Option<(i64, i64)> {
        let l = self.len();
        let last_2nd = self.get(l - 2);
        let last = self.get(l - 1);
        match (last_2nd, last) {
            (Some(a), Some(b)) => Some((*a, *b)),
            _ => None,
        }
    }


    fn mut1(&mut self, f: fn(i64) -> Vec<i64>) -> StackResponse {
        if let Some(a) = self.pop() {
            self.remove_and_add(0, &mut f(a))
        } else {
            UnderFlow
        }
    }

    fn mut2(&mut self, f: fn(i64, i64) -> Vec<i64>) -> StackResponse {
        if let Some((a, b)) = self.last2() {
            self.remove_and_add(2, &mut f(a, b))
        } else {
            UnderFlow
        }
    }

    fn mut1_err(&mut self, f: fn(i64) -> Result<Vec<i64>, String>) -> StackResponse {
        if let Some(a) = self.pop() {
            match f(a) {
                Ok(mut vec) => self.remove_and_add(0, &mut vec),
                Err(error) => Error(error)
            }
        } else {
            UnderFlow
        }
    }

    fn mut2_err(&mut self, f: fn(i64, i64) -> Result<Vec<i64>, String>) -> StackResponse {
        if let Some((a, b)) = self.last2() {
            match f(a, b) {
                Ok(mut vec) => self.remove_and_add(2, &mut vec),
                Err(error) => Error(error)
            }
        } else {
            UnderFlow
        }
    }
}

fn main() {
    let mut output = vec!["Stack-push machine ready!".to_string()];


    let asker = |x: &str| -> Result<Vec<Token>, String> {
        if x.to_ascii_lowercase() == "exit".to_string() {
            println!("exiting...");
            exit(0);
        }
        x.split_ascii_whitespace().map(|x| {
            match x.parse().map(|x| Token::Value(x)) {
                Ok(e) => Ok(e),
                Err(_) => match x {
                    "*" => Ok(Multiply),
                    "/" => Ok(Divide),
                    "+" => Ok(Add),
                    "-" => Ok(Minus),
                    "mod" => Ok(Modulo),
                    "." => Ok(Pop),
                    "swap" => Ok(Swap),
                    ".s" => Ok(Stack),
                    "negate" => Ok(Negate),
                    "abs" => Ok(Absolute),
                    "max" => Ok(Max),
                    "min" => Ok(Min),
                    "dup" => Ok(Duplicate),
                    "drop" => Ok(Token::Drop),
                    "nip" => Ok(Nip),
                    _ => Err("Unknown token: ".to_owned() + x)
                }
            }
        }).collect()
    };

    let mut stack: Vec<i64> = Vec::new();


    loop {
        output.insert(0, "=>".to_string());
        let tokens = ask_for_input(&output.join(" "), asker);
        output.clear();
        let mut error = false;
        for i in tokens.iter() {
            let resp = match i {
                Token::Value(e) => stack.remove_and_add(0, &mut vec![*e]),
                Multiply => stack.mut2(|a, b| vec![a * b]),
                Divide => stack.mut2_err(|a, b| match b {
                    0 => Err("divide by zero".to_string()),
                    _ => Ok(vec![a / b]),
                }),
                Add => stack.mut2(|a, b| vec![a + b]),
                Minus => stack.mut2(|a, b| vec![a - b]),
                Modulo => stack.mut2_err(|a, b| match b {
                    0 => Err("modulo by zero".to_string()),
                    _ => Ok(vec![a % b]),
                }),
                Pop => {
                    match stack.pop() {
                        Some(e) => OK(e.to_string()),
                        None => UnderFlow
                    }
                }
                Swap => stack.mut2(|a, b| vec![b, a]),
                Stack => OK(
                    stack
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                ),

                Negate => stack.mut1(|a| vec![-a]),
                Absolute => stack.mut1(|a| vec![a.abs()]),
                Max => stack.mut2(|a, b| vec![a.max(b)]),
                Min => stack.mut2(|a, b| vec![a.min(b)]),
                Duplicate => stack.mut1(|a| vec![a, a]),
                Token::Drop => stack.mut1(|_| vec![]),
                Nip => stack.mut2(|_, b| vec![b]),
            };
            match resp {
                OK(msg) => output.push(msg),
                Accepted => {}
                Error(err) => {
                    output.push(err);
                    error = true;
                    break;
                }
                UnderFlow => {
                    output.push(String::from("stack underflow"));
                    error = true;
                    break;
                }
            }
        };
        if !error {
            output.push("Ok".to_string())
        }
    }
}



