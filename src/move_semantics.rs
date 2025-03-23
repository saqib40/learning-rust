pub fn move1() {
    let s1 = String::from("hello");
    let s2 = s1; // below statement will work if s1.clone()
    //println!("{}", s1); // This line would cause a compile error because ownership has been moved.
    println!("{}", s2); // works fine
    move2();
    move3();
}

fn move2() {
    let my_string = String::from("hello");
    takes_ownership(my_string);
    //println!("{}", my_string); // This line would cause a compile error because ownership has been moved.
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string); // `some_string` now owns the data.
}

// rather than cloning
// there's one more workaround, not the best one though
fn move3() {
    let mut s1 = String::from("hello");
    s1 = takes_ownership2(s1);
    println!("{}", s1);
}

fn takes_ownership2(some_string: String) -> String {
    println!("{}", some_string); 
    return some_string; // return the string ownership back to the original main fn
}