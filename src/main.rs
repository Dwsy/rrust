#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn main() {
    let mut data = Vec::new();
    data.push(1);
    data.push(2);
    data.push(3);

    let f1 = File {
        name: String::from("f1.txt"),
        data: data,
    };

    let f1_name = &f1.name;
    let f1_length = f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}