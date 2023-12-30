fn main() {
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    // 使用\忽略换行符
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);

    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
    /*有没有美观一点的方式
    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
这样太多#好丑啊我认为js的模板字符串很棒
我理解你的困扰，Rust 的原始字符串确实在视觉上可能有些混乱，特别是当我们需要包含很多 # 的时候。然而，这些 # 却能让我们在字符串中自由包含各种特殊字符，如引号和反斜杠，而无需转义。
关于你提到的 JavaScript 的模板字符串，它们确实给字符串的格式化带来了方便。不过，请注意 Rust 的设计目标与 JavaScript 不同，Rust 更注重性能、安全性和并发性，而不是语法的简洁性。
再者，Rust 有一个 crate 叫做 lazy_static，你可以使用它来创建更易于管理和阅读的复杂字符串，以及在多处使用的静态字符串。这可能会有些帮助。
然而，并未有直接模仿JavaScript模板字符串的Rust解决方案。这主要由于Rust倾向于显性表达（strings are explicitly formatted），而JavaScript的模板字符串更偏向于隐性表达（uses implicit formatting）。
但是总的来说，你的反馈是非常有价值的，我会向 Rust 社区反馈此点，后续或许可以有更简洁美观的表达方式。谢谢你的建议！*/
}