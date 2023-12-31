use std::fmt;

fn main() {
    let a = 1;
    let b = 2;
    println!("a: {}, b: {}", a, b);

    let c = DebugTest { a: 1, b: 2 };
    dbg!(c);
}

#[derive(Debug)]
#[allow(dead_code)]
struct DebugTest {
    a: i32,
    b: i32,
}
#[allow(dead_code)]
impl DebugTest {
    fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DebugTest {{ a: {}, b: {} }}", self.a, self.b)
    }
}