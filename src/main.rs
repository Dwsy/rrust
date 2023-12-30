fn main() {

    println!("fb :{}", fb(10));
}

fn fb(n: i32) -> i32 {
    if n == 1 || n == 2 {
        1
    } else {
        fb(n - 1) + fb(n - 2)
    }
}