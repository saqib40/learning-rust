use std::collections::HashMap;

pub fn example() {
    // creation
    let mut scores : HashMap<&str,i32> = HashMap::new();
    // insertion
    scores.insert("Alice", 50);
    scores.insert("Bob", 80);
    // printing
    println!("{:?}", scores);
    // accessing
    if let Some(score) = scores.get("Alice") {
        println!("Alice's score: {}", score);
    }
    // iteration
    for (key, value) in scores {
        println!("{}: {}", key, value);
    }
}