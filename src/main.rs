use std::io::stdin;

const PHRASE: [&str; 12] = ["A partridge in a pear tree", "Two turtle doves",
    "Three French hens", "Four calling birds", "Five gold rings",
    "Six geese a laying", "Seven swans a swimming", "Eight maids a milking",
    "Nine ladies dancing", "Ten lords a leaping", "Eleven pipers piping",
    "Twelve drummers drumming",
];

const ORDINAL: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth",
    "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

fn ask_for_days() -> usize {
    loop {
        println!("How many days do you want: ");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Cannot read line");

        match input.trim().parse() {
            Ok(e) => break match e {
                1..=12 => e,
                _ => {
                    println!("Must be from 1 to 12");
                    continue;
                }
            }
            ,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }
    }
}

fn main() {
    loop {
        let mut x = 0;
        let end = ask_for_days();
        while x < end {
            println!("On the {} of Christmas my true love gave to me", ORDINAL[x]);
            println!("{}", day(x));
            x += 1;
        }
    }
}

fn day(day: usize) -> String {
    match day {
        0 => PHRASE[0].to_string(),
        1 => (PHRASE[1].to_string() + " and " + PHRASE[0].to_ascii_lowercase().as_str()).to_string(),
        _ => {
            let phrases: Vec<_> = PHRASE[1..(day)].iter().map(|x| { x.to_ascii_lowercase() }).rev().collect();
            let p: String = phrases.join(", ");
            PHRASE[day].to_string() + ", " + p.as_str() + " and " + PHRASE[0].to_ascii_lowercase().as_str()
        }
    }
}



