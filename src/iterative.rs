pub fn iterative() {
    let mut number = 3;
    while number > 0 {
        println!("Number: {}", number);
        number -= 1;
    }

    for i in 1..5 { // Range 1 to 4
        println!("i: {}", i);
    }
    for x in 1..=5 { // Range 1 to 5
        println!("x: {}", x);
    }

    let mut count = 0;
    loop {
        println!("Count: {}", count);
        count += 1;
        if count == 5 {
            break;
        }
    }
}