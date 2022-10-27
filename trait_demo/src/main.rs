// 定义 Trait
pub trait Summary {
    fn summarize(&self) -> String;
}
// 定义结构体 NewsArticle
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}
// 为结构体 NewsArticle 实现 Summary Trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
// 定义结构体 Tweet
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}
// 为结构体 Tweet 实现 Summary Trait
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
fn main() {
    // 初始化结构体 Tweet
    let tweet = Tweet {
        username: String::from("Jartto"),
        content: String::from("of course, as you probably already know, people."),
        reply: false,
        retweet: false
    };
    // 调用 trait 定义的公用方法 summarize
    println!("1 new tweet: {}", tweet.summarize());
}

// Error: no method named `summarize` found for struct `Tweet` in the current scope
// https://github.com/rust-lang/book/issues/1018
