use std::ops::Add;

#[derive(Debug)]
struct User {
    name: String,
    seconds: i32,
}

impl User {
    fn new(name: String, s: i32) -> Self {
        User {
            name,
            seconds: s,
        }
    }

    fn add(&self, seconds1: i32) -> String {
        let s = self.seconds + seconds1;
        let name = format!("{}{}", self.name, s);
        name
    }
}

impl Add<i32> for User {
    type Output = User;

    fn add(self, rhs: i32) -> Self::Output {
        User {
            name: self.name,
            seconds: self.seconds + rhs,
        }
    }
}

fn main() {
    let name = String::from("Dwsy");
    let user = User::new(name, 18);
    let user = user + 1;
    println!("{:?}", user);
}
