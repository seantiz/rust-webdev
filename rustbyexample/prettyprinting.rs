#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    address: Address,
}

#[derive(Debug)]
struct Address {
    street: String,
    city: String,
    country: String,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
        address: Address {
            street: String::from("123 Main St"),
            city: String::from("Anytown"),
            country: String::from("USA"),
        },
    };

    println!("Regular Debug:");
    println!("{:?}", person);

    println!("\nPretty Printed Debug:");
    println!("{:#?}", person);
}
