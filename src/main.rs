fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);
    // let r1 = &mut s1;
    // let r2 = &mut s1;
    // println!("{}, {}", r1, r2);
    // 可变引用同时只能存在一个
    // 不过可变引用并不是随心所欲、想用就用的，它有一个很大的限制： 同一作用域，特定数据只能有一个可变引用：
    println!("{}", s1);
}

fn calculate_length(s: &String) -> usize {
   return  s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}