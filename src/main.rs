use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // 打开文件，如果文件不存在则创建它
    let mut file = File::create("hello.txt")?;

    // 定义要写入的字符串
    let contents = "Hello, world!";

    // 将字符串写入文件
    file.write_all(contents.as_bytes())?;

    Ok(())
}
