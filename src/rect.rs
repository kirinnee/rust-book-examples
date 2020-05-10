use crate::Shape;

pub struct Rectangle {
    width: u64,
    height: u64,
}

impl Rectangle {
    pub(crate) fn new(width: u64, height: u64) -> Rectangle {
        Rectangle { width, height }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> u64 {
        self.width * self.height
    }

    fn perimeter(&self) -> u64 {
        self.width * 2 + self.height * 2
    }

    fn print(&self) {
        println!();
        for _ in 0..self.height {
            for _ in 0..self.width {
                print!("**")
            }
            println!();
        }
        println!();
    }
}

