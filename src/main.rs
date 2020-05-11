use std::fmt::Debug;
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

fn main() {
    let ask = |x: &str| -> Result<(), String>{
        let v: Vec<_> = x.split_ascii_whitespace().collect();
        match v.as_slice() {
            ["point", x, y] => {
                match (x.parse(), y.parse()) {
                    (Ok(x), Ok(y)) => Ok(Point { x, y }.outline_print()),
                    _ => Err("Not a point".to_string()),
                }
            }
            ["color", r, g, b] => {
                match (r.parse(), g.parse(), b.parse()) {
                    (Ok(r), Ok(g), Ok(b)) => Ok(Color(r, g, b).outline_print()),
                    _ => Err("Not a color".to_string())
                }
            }
            ["point3d", x, y, z] => {
                match (x.parse(), y.parse(), z.parse()) {
                    (Ok(x), Ok(y), Ok(z)) => Ok(Point3D { x, y, z }.outline_print()),
                    _ => Err("Not a point 3d".to_string())
                }
            }
            _ if v[0] == "vec" => {
                let y = (&v[1..]).to_vec();
                Ok(y.outline_print())
            }
            _ => Err("Unknown command".to_string()),
        }
    };
    loop {
        ask_for_input("Command:", ask);
    }
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Color(u32, u32, u32);

#[derive(Debug)]
struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}

impl OutlinePrint for Point {}

impl OutlinePrint for Color {}

impl OutlinePrint for Point3D {}

impl<T> OutlinePrint for Vec<T> where T: Debug {}

trait OutlinePrint: Debug {
    fn outline_print(&self) {
        let output = format!("{:?}", self);
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}




