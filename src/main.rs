// FIX the errors
fn main() {
    let mut vec1 = Vec::with_capacity(2);
    let mut vec2 = Vec::with_capacity(2);
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);
    vec1.push(4);

    vec2.push(5);
    vec2.push(6);
    vec2.push(7);
    vec2.push(8);

    let slice3 = &mut vec1[0..];


    unsafe {
        let slice4 = slice3.get_unchecked_mut(0..8);
        println!("{:?}", slice4);

        println!("{:p},{:p}", &vec1,&vec2);
    }
}