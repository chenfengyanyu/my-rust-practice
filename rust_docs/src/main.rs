/// 将传入的数字加 1
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_create::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
fn main() {
    add_one(5);
}

// cargo doc 会调用 Rust 内置的 rustdoc 工具在 target/doc 路径下生成 HTML 文档
// cargo doc --open 生成并自动在浏览器中打开当前包的文档（以及所有依赖包的文档）

// 如果文档显示有问题，可以启动一个 HTTP Server 托管服务
// /Users/jartto/Documents/Project/my-rust-practice/rust_docs/target/doc
// 执行 http-server