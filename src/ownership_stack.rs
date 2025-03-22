pub fn ownership_stack() {
    let x = 5;
	let y = x; // unused variable warning
	println!("{}", x);
    example2();
}

fn example2() {
    let x = 1; // crated on stack
    let y = 3; // created on stack
    println!("{}", sum(x, y));
    println!("Hello, world!");
}

fn sum(a: i32, b: i32) -> i32 {
let c = a + b;
return c;
}