pub fn conditionals() {
    let number = 7;

    if number < 0 {
        println!("Negative");
    } else if number == 0 {
        println!("Zero");
    } else {
        println!("Positive");
    }

    let day = 3;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        _ => println!("Other day"),
    }
}