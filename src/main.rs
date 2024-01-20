fn main() {
    println!("{}", max(&1, &2));
}

fn max<'a, T: PartialOrd>(foo: &'a T, bar: &'a T) -> &'a T {
    if foo > bar {
        foo
    } else {
        bar
    }
}
