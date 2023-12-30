use std::ops::Add;

fn main() {
    let mut s = String::from("Hello rust!");
    s.insert(5, ',');
    println!("插入字符 insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", s);

    let mut s = String::from("高桥李依");

    for char in  ",Rie".chars() {
        s.insert(s.len(), char);
        println!("插入字符 insert() -> {}", s);
    }

    let string_replace = "I like rust. Learning rust is my favorite! I like rust. Learning rust is my favorite!";
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replacen);

    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);

    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    string_remove.remove(0);
    // dbg!(string_remove);
    // 下面代码会发生错误，因为第二个汉字占 3 个字节，而 rust 的 remove 方法只能删除一个字节
    // string_remove.remove(1);
    // 直接删除第二个汉字
    string_remove.remove(3);
    dbg!(string_remove);

    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let result = string_append.clone() + &string_rust;
    let rusult2 = string_append.add(&string_rust);
    /*Rust 语言中，对于数据类型的所有权进行了严格的管理和规定。具体来说，当你执行 let result = string_append + &string_rust; 这一行代码时，
    你实际上是在调用 String 类型的 add 方法，而这个方法会消耗（take ownership）调用它的 String（此处为 string_append）。
    也就是说，在调用完 add 之后，string_append 就不再有效，它的所有权已经被转移走了。
    然而，当你试图执行下一句 let result2 = string_append.add(&string_rust); 时，你会发现 string_append 已经不能再用了，因为它已经被上一个操作消耗掉了。
    在 Rust 中，这被称为“移动语义（move semantics）”。
    所以在这个例子中，你需要在第一次操作中用 clone() 复制一份 string_append，使得原本的 string_append 并没有被消耗掉，而是它的副本被使用并消耗，这
    样你就可以在接下来的操作中继续使用 string_append 了。使用 clone 方法可以做到这一点，因为 clone 方法会为调用者创建一个完全相同数据的新副本，而不会消耗调用它的变量。*/
    // println!("{}", string_append);
    // println!("{}", string_rust);
    // let mut result = result + "!"; // `result + "!"` 中的 `result` 是不可变的
    // result += "!!!";

    println!("连接字符串 + -> {}", result);
    println!("连接字符串 add() -> {}", rusult2);

    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    //format 不会转移所有权
    println!("{}", s1);

    println!("格式化字符串 format() -> {}", s);
}