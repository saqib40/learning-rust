// defining the trait
trait Summary {
    fn summarize(&self) -> String;
    // a default implementation
    fn eval(&self) -> String {
        return String::from("hii this is a default implementation");
    }
}

// defining the struct
struct User {
    name : String,
    age : u32
}

// implementing trait on struct
impl Summary for User {
    fn summarize(&self) -> String {
        return format!("User {} is {} years old", self.name, self.age);
    }
}

// traits as parameters
fn notify(item : &impl Summary) {
    println!("Breaking news {}", item.summarize());
}
// another way is trait bound syntax
fn notify2<T : Summary>(item : &T) {
    println!("Breaking news {}", item.summarize());
}

// defining struct and using it's function
pub fn ex1() {
    let user = User {
        name: String::from("Saqib"),
        age: 22
    };
    println!("{}", user.summarize());
    println!("{}", user.eval());
    notify(&user);
    notify2(&user);
}