use crate::Shape;

pub struct Square {
    width: u64,
}

impl Square {
    pub fn new(width: u64) -> Square {
        Square { width }
    }
}

impl Shape for Square {
    fn area(&self) -> u64 {
        self.width * self.width
    }

    fn perimeter(&self) -> u64 {
        self.width * 4
    }

    fn print(&self) {
        println!();
        for _ in 0..self.width {
            for _ in 0..self.width {
                print!("**")
            }
            println!();
        }
    }
}
