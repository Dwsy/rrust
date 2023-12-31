use std::fmt;

fn main() {
    let a = 1;
    let b = 2;
    println!("a: {}, b: {}", a, b);

    let c = DebugTest { a: 1, b: 2 };
    println!("c: {:?}", c);
    println!("c: {:#?}", c);

}

struct DebugTest {
    a: i32,
    b: i32,
}
impl fmt::Debug for DebugTest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DebugTest {{ a: {}, b: {} }}", self.a, self.b)
    }
}