use std::io::stdin;

fn main() {
    let ordinal = ["first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
}

fn day(day: usize) -> String {
    let phrase = ["A partridge in a pear tree", "Two turtle doves",
        "Three French hens", "Four calling birds", "Five gold rings",
        "Six geese a laying", "Seven swans a swimming", "Eight maids a milking",
        "Nine ladies dancing", "Ten lords a leaping", "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    match day {
        0 => phrase[0].to_string(),
        1 => (phrase[1].to_string() + " and " + phrase[2].to_ascii_lowercase())
    }
}



