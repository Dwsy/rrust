use std::ops::{Add, Sub, Mul, Div};

enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

fn calculate<T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T>>(a: T, b: T, operator: Operator) -> T {
    match operator {
        Operator::Add => a + b,
        Operator::Sub => a - b,
        Operator::Mul => a * b,
        Operator::Div => a / b,
    }
}

fn main() {
    println!("{}", calculate(10, 5, Operator::Add));
    println!("{}", calculate(10, 5, Operator::Sub));
    println!("{}", calculate(10, 5, Operator::Mul));
    println!("{}", calculate(10, 5, Operator::Div));
}