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