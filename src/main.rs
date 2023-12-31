pub trait Summary {
    fn summarize(&self) -> String;
    fn new() -> Self;
}

pub struct Post {
    pub title: String,
    // 标题
    pub author: String,
    // 作者
    pub content: String, // 内容
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
    fn new() -> Self {
        Post {
            title: "".to_string(),
            author: "".to_string(),
            content: "".to_string(),
        }
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }

    fn new() -> Self {
        todo!()
    }
}

fn main() {
    let mut post = Post::new();
    post = Post {
        title: String::from("Rust从入门到入门"),
        author: "author".to_string(),
        content: "content".to_string(),
    };

    println!("{}", post.summarize());
}