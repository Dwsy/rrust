fn main() {
    let user = User::new("Dwsy".to_string(), 18);
    println!("name: {}", user.get_name());
    println!("{}", user.age);
    println!("{}", user.name);
}

struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: String, age: u32) -> User {
        User {
            name,
            age,
        }
    }
}
impl User {
    fn get_name(&self) -> &String {
        &self.name
    }
}