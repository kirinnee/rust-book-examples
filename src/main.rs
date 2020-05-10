use std::io::stdin;
use std::process::exit;

use crate::rect::Rectangle;
use crate::square::Square;

mod square;
mod rect;

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

trait Shape {
    fn area(&self) -> u64;
    fn perimeter(&self) -> u64;
    fn print(&self);
}

enum Operations {
    Rectangle(Rectangle),
    Square(Square),
    List,
}

fn main() {
    let mut shapes: Vec<Box<dyn Shape>> = vec![];
    let ask = |x: &str| -> Result<Operations, String> {
        let vec: Vec<_> = x.split_ascii_whitespace().collect();
        match vec.as_slice() {
            ["sq", w] => match w.parse() {
                Ok(w) => Ok(Operations::Square(Square::new(w))),
                Err(err) => Err(err.to_string()),
            },
            ["rect", w, h] => match (w.parse(), h.parse()) {
                (Ok(w), Ok(h)) => Ok(Operations::Rectangle(Rectangle::new(w, h))),
                _ => Err("Cannot parse string".to_string()),
            },
            ["list"] => Ok(Operations::List),
            ["exit"] => exit(0),
            _ => Err("Unknown operation".to_string()),
        }
    };

    loop {
        let operation = ask_for_input("Command: ", ask);

        match operation {
            Operations::Rectangle(rect) => shapes.push(Box::new(rect)),
            Operations::Square(square) => { shapes.push(Box::new(square)) }
            Operations::List => print_shapes(&shapes),
        }
    }
}

fn print_shapes(shapes: &Vec<Box<dyn Shape>>) {
    for shape in shapes {
        shape.print();
        println!("Area: {}, Perimeter: {}", shape.area(), shape.perimeter());
    }
}




