use std::thread;

fn ex1() {
    let handle = thread::spawn(|| {
        println!("Hello from another thread!");
    });

    println!("Hello from main thread!");

    handle.join().unwrap(); // Waits for the spawned thread to finish
}

fn ex2() {
    // Thread Panic & Error Handling
    // If a thread panics, it returns an error on .join()
    let handle = thread::spawn(|| panic!("Oops!"));
    match handle.join() {
        Ok(_) => println!("Thread finished successfully."),
        Err(e) => println!("Thread panicked: {:?}", e),
    }
}

fn ex3() {
    // move keyword is required to transfer ownership into the thread closure
    let data = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Data: {:?}", data);
    });
}
