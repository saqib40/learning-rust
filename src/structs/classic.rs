// Most commonly used for storing structured data
// Each field has a name and a data type

struct Person {
    name: String,
    age: u8,
    city: String,
}

pub fn classic() {
    let person = Person {
        name: String::from("Saqib"),
        age: 22,
        city: String::from("Bangalore"),
    };
    println!("{} is {} years old from {}.", person.name, person.age, person.city);
}