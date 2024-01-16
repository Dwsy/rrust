// FILL in the blanks
fn main() {
    // get a slice of String with reference: String -> &str
    let mut s = String::from("hello, world");

    let slice1: &str = s.as_mut(); // in two ways
    assert_eq!(slice1, "hello, world");

    let slice2 = &s[0..5];
    assert_eq!(slice2, "hello");

    //Note! The type here cant be `&mut str` due to `push` is ONLY defined on String type and its mut reference: `&mut String` !
    // So you can't use `s.as_mut_str()`
    let slice3: &mut String = &mut s;
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");

    s=String::from("hello, world new");
    println!("{}", s);
    slice3.push_str(" new");

    println!("Success!")
}