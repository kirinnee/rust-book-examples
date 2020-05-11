use std::io::stdin;

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
struct Color(u32);


fn transform(i: u32) -> u64 {
    i as u64 + i as u64
}

fn main() {
    let vec: Vec<_> = (0u32..100).map(Color).collect();
    println!("{:?}", vec);

    let vec: Vec<_> = (0u32..100).map(transform).collect();
    println!("{:?}", vec);


}



