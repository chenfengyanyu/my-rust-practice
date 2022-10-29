// 泛型生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x   
    } else {
        y
    }
}

fn main() {
    let str1 = String::from("abcd");
    let str2 = "xyz";

    let res = longest(str1.as_str(), str2);
    println!("The longest string is {}", res);
}

// 借用检查器无法知道 x 和 y 的声明周期是如何与返回值的声明周期关联的
// error[E0106]: missing lifetime specifier
//  --> src/main.rs:2:33
//   |
// 2 | fn longest(x: &str, y: &str) -> &str {
//   |               ----     ----     ^ expected named lifetime parameter
//   |
//   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
// help: consider introducing a named lifetime parameter
//   |
// 2 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//   |           ++++     ++          ++          ++

// For more information about this error, try `rustc --explain E0106`.