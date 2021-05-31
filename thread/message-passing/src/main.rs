use std::sync::mpsc;
use std::thread;
use std::time::Duration;


fn main() {
    
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("1 hi"),
            String::from("1 from"),
            String::from("1 the"),
            String::from("1 thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("0 more"),
            String::from("0 messages"),
            String::from("0 for"),
            String::from("0 you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(10));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
