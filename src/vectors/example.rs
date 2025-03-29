// write a function that takes a vec as i/p
// and returns a vector with even values as o/p

// method 1
// move syntax ; i.e moving the owner
fn filter1(vec : Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::new();
    for num in vec {
        if(num%2) == 0 {
            ans.push(num);
        }
    }
    return ans;
}

pub fn example1() {
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Example 1");
    println!("{:?}", filter1(numbers));
}

// method 2
// immutable reference
fn filter2(vec : &Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::new();
    for num in vec {
        if(*num%2) == 0 {
            ans.push(*num);
        }
    }
    return ans;
}

pub fn example2() {
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Example 2");
    println!("{:?}", filter2(&numbers));
}

// method 3
// mutable reference
fn filter3(vec : &mut Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::new();
    for num in vec {
        if(*num%2) == 0 {
            ans.push(*num);
        }
    }
    return ans;
}

pub fn example3() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("Example 3");
    println!("{:?}", filter3(&mut numbers));
}