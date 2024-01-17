fn main() {
    let mut array = [1, 2, 3, 4, 5];
    let ptr = array.as_mut_ptr();
    println!("ptr = {:p}", ptr);
    let add1 = unsafe { ptr.add(0) };
    println!("add1 = {:p}", add1);

    unsafe {
        *add1 = 6;
        println!("array[5] = {}", *ptr);
    }
}
