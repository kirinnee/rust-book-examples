use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::io::stdin;

fn ask_for_input<F, T>(question: &str, f: F) -> T where F: (Fn(&str) -> Result<T, Box<dyn Error>>) {
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
struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn perimeter(&self) -> u32 {
        self.height * 2 + self.width * 2
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\tWidth: {}\n\tHeight: {}\n\tArea: {}\n\tPerimeter: {}", self.width, self.height, self.area(), self.perimeter())
    }
}


fn request_rect() -> Rectangle {
    let obtain_u32 = |x: &str| {
        x.parse::<u32>().map_err(|e| e.into())
    };
    let height = ask_for_input("Width: ", obtain_u32);
    let width = ask_for_input("Height: ", obtain_u32);
    Rectangle::new(width, height)
}


fn main() {
    loop {
        println!();
        println!("Please key in the parameters for rectangle 1: ");
        let r1 = request_rect();
        println!("Please key in the parameters for rectangle 2: ");
        let r2 = request_rect();

        println!("Rect 1: \n{}", r1);
        println!("Rect 2: \n{}", r2);

        println!("Rect 1 {} hold Rect 2", if r1.can_hold(&r2) { "can" } else { "cannot" });
    }
}




