fn main() {
    let sum:i32 = {
        let mut i  = 0;
        for j in 0..101 {
            i += j
        }
         i
    };

    println!("1..100 sum eq : {}", sum);

}