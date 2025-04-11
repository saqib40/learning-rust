use std::sync::mpsc;
use std::thread;

fn ex4() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap(); // transfer ownership
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn ex5() {
    let (tx, rx) = mpsc::channel();
    for i in 0..5 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(format!("thread {}", i)).unwrap();
        });
    }
    for received in rx {
        println!("Got: {}", received);
    }
}