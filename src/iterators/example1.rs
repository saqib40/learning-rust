pub fn iter_ex1() {
    let v = vec![1, 2, 3];
    let mut iter = v.iter(); // Create an iterator

    println!("{:?}", iter.next()); // Some(1)
    println!("{:?}", iter.next()); // Some(2)
    println!("{:?}", iter.next()); // Some(3)
    println!("{:?}", iter.next()); // None
}

pub fn iter_ex2() {
    let mut v = vec![1, 2, 3];
    // Immutable Iterator
    for num in v.iter() {
        println!("Immutable: {}", num);
    }
    // Mutable Iterator
    for num in v.iter_mut() {
        *num *= 2;
        println!("Mutable: {}", num);
    }
    // Consuming Iterator
    for num in v.into_iter() {
        println!("Consuming: {}", num);
    }
    
    //Write the logic to first filter all odd values then double each value and create a new vector
    let v1 : Vec<i32> = vec![4,5,6,7,8,9,10];
    let v2 : Vec<i32> = v1.iter().filter(|x| *x%2==0).map(|x| x*2).collect();
    println!("{:?}", v2);
}