use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx1, rx) = mpsc::channel();

    let tx2 = Sender::clone(&tx1);

    thread::spawn(move || {
        let vals: Vec<_> = vec!["hi", "this", "is", "ernest"]
            .iter()
            .map(|x| (*x).to_string())
            .collect();
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals: Vec<_> = vec!["hi", "this", "is", "cc"]
            .iter()
            .map(|x| (*x).to_string())
            .collect();

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}



