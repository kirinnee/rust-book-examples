use std::cmp::Ordering;
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

#[derive(PartialEq, PartialOrd)]
struct Point<S, T> {
    x: S,
    y: T,
}

impl<T: PartialOrd, U: PartialOrd> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }

    fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    }
}

fn main() {
    let world = ask_for_input("Enter your name: ", |x| Ok(x.to_string()));

    let p1 = Point::new("Hello ".to_string(), 10);
    let p2 = Point::new(50.7, world + "!");

    let Point { x, y } = p1.mixup(p2);

    println!("{}{}", x, y);
}

