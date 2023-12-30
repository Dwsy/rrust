fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    let temp = &[2, 3];
    println!("{:?}", slice); // [2, 3]
    println!("{:?}", temp); // [2, 3]
    println!("{:?}", slice.as_ptr());
    println!("{:?}", temp.as_ptr());
    println!("-------------------");
    println!("{:?}", slice.as_ptr_range());
    println!("{:?}", temp.as_ptr_range());

    assert_eq!(*slice, *temp);
}