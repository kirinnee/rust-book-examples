use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("number {:?} from ", v);
            thread::sleep(Duration::from_millis(300));
        }
    });


    handle.join().unwrap();
    for i in 1..5 {
        println!("number {} from ", i);
        thread::sleep(Duration::from_millis(500));
    }
}


