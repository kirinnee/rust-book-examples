use std::io::stdin;
use std::ops::Deref;

use crate::List::{Cons, Nil};

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

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}


#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let b = MyBox::new(5);
    println!("b = {}", *b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("Cons list: {:?}", list)
}



