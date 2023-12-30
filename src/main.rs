fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(&x, y);

    println!("x mem_addr = {:p}", &x);
    println!("y value mem_addr = {:p}", y);
    println!("y mem_addr = {:p}", &y);
}
