// FIX the errors
    fn main() {
        let mut v = vec![1, 2, 3];
        let mut v2 = vec![1, 2, 3];

        println!("v1 p:{:p} v2 p:{:p}", v.as_ptr(), v2.as_ptr());

        let slice1 = &v[..];
        // out of bounds will cause a panic
        // You must use `v.len` here
        let slice2 = &v[0..v.len()];

        assert_eq!(slice1, slice2);

        // slice are read only
        // Note: slice and &Vec are different
        let vec_ref: &mut Vec<i32> = &mut v;
        (*vec_ref).push(4);
        v2.push(888);
        let slice3 = &mut v[0..];

        assert_eq!(slice3, &[1, 2, 3, 4]);

        unsafe {
            let slice4 = slice3.get_unchecked_mut(0..100);
            println!("{:?}", slice4);
        }
        println!("{:?}", v2);

        println!("\n-------");
        println!("v1 p:{:p} v2 p:{:p}", v.as_ptr(), v2.as_ptr());
        println!("Success!")
}