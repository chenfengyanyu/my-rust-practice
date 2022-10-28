use std::fmt::{Debug, Display};

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
    pub content: String,
}
// 为结构体 NewsArticle 实现 Summary Trait
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

// 方式一：impl Trait 作为参数
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 方式二：trait 约束——trait bound
pub fn notify2<T: Summary>(item: T) {
    println!("T info! {}", item.summarize());
}

fn main() {
    // 初始化结构体 NewsArticle
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Jartto"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };
    // 调用 trait 定义的公用方法 summarize
    println!("New article available! {}", article.summarize());

    // 方式一：传入 NewsArticle 结构体作为参数
    // notify(article);

    // 方式二：泛型约束
    notify2(article);
}

// 补充学习
// 通过+语法来指定多个 trait 约束
fn notify(item: impl Summary + Display) {}
// 复杂形式
fn notify<T: Summary + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}
// 使用 where 简化 trait 约束
fn notify<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // Todo...
}
