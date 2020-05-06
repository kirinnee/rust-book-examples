#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let expected1 = Rectangle { width: 5, height: 10 };
        let expected2 = Rectangle { width: 700, height: 900 };

        let subj1 = Rectangle::new(5, 10);
        let subj2 = Rectangle::new(700, 900);

        assert_eq!(expected1, subj1);
        assert_eq!(expected2, subj2);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle::new(8, 7);
        let smaller = Rectangle::new(2, 1);
        assert!(!smaller.can_hold(&larger));
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 200.")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got -1.")]
    fn smaller_than_0() {
        Guess::new(-1);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        let y = 2 + 2;
        if y == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

#[derive(PartialEq, Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}