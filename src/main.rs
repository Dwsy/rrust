fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut s = String::new();
    s.push_str("hello");

    let v = vec![104, 101, 108, 108, 111];

    // 将字节数组转换成 String
    let s1 = String::from_utf8(v)?;

    assert_eq!(s, s1);

    println!("Success!");

    Ok(())
}
