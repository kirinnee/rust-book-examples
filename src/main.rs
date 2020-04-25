use std::io::stdin;

fn main() {
    let ordinal = ["first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    let mut x = 0;
    while x < 12 {
        println!("On the {} of Christmas my true love gave to me", ordinal[x]);
        println!("{}", day(x));
        x += 1;
    }
}

fn day(day: usize) -> String {
    let phrase = vec!["A partridge in a pear tree", "Two turtle doves",
                      "Three French hens", "Four calling birds", "Five gold rings",
                      "Six geese a laying", "Seven swans a swimming", "Eight maids a milking",
                      "Nine ladies dancing", "Ten lords a leaping", "Eleven pipers piping",
                      "Twelve drummers drumming",
    ];

    match day {
        0 => phrase[0].to_string(),
        1 => (phrase[1].to_string() + " and " + phrase[0].to_ascii_lowercase().as_str()).to_string(),
        _ => {
            let phrases: Vec<_> = phrase[1..(day)].iter().map(|x| { x.to_ascii_lowercase() }).rev().collect();
            let p: String = phrases.join(", ");
            phrase[day].to_string() + ", " + p.as_str() + " and " + phrase[0].to_ascii_lowercase().as_str()
        }
    }
}



