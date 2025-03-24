pub fn unrecoverable() {
    let num = 10;
    if num == 10 {
        panic!("This is an unrecoverable error!");
    }
    println!("This line will not be printed.");
}
