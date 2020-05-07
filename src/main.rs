use std::io::stdin;
use std::rc::Rc;

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

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

impl List {
    fn nil() -> Rc<List> {
        Rc::new(Nil)
    }
    fn new(i: i32, next: &Rc<List>) -> Rc<List> {
        Rc::new(Cons(i, Rc::clone(next)))
    }
}



fn main() {
    let a = List::new(1, &List::nil());
    let b = List::new(2, &a);
    println!("created b, count {}", Rc::strong_count(&b));
    let c = List::new(3, &b);
    println!("created c, count {}", Rc::strong_count(&b));
    let d = List::new(4, &b);
    println!("created d, count {}", Rc::strong_count(&b));


    {
        let e = List::new(5, &b);
        println!("created e, count {}", Rc::strong_count(&b));
    }
    println!("deleted e, count {}", Rc::strong_count(&b));
}



