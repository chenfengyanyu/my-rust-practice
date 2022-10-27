// 定义 Trait 默认实现
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("{}", self.summarize_author())
    }
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
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

fn main() {
    // 初始化结构体 NewsArticle
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Jartto"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    // 调用 trait 定义的公用方法 summarize
    println!("New article available! {}", article.summarize());
}

