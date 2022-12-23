#[macro_export] // 意味着这个宏会在他所处的包被引入作用域后可用
// macro_rules 定义宏
macro_rules! vec {
    // 宏模式匹配的是 Rust 代码结构，而不是值
    // $x:expr 可以匹配任意的 Rust 表达式，并将其命名为 $x
    // $() 之后的逗号意味着一个可能的字面逗号分隔符会出现在捕获代码的后面
    // 而逗号后的 * 则意味着这个模式能够匹配零个或多个 * 之前的东西
    // 调用 vec![1, 2, 3]时，$x 模式会分别匹配 3 个表达式：1、2 及 3
    ($( $x:expr ),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
fn main() {
    println!("Hello, world!");
}
