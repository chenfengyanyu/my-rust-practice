use std::fs::File;
fn main() {
    // 当 Result 的返回值是 Ok 变体时，unwrap 就会返回 Ok 内部的值
    // 当 Result 的返回值是 Err 变体时，unwarp 则调用 panic 宏
    // let f = File::open("hello.txt").unwrap();

    // expect 触发 panic 时会将传入的参数字符串作为错误提示信息输出
    // 而 panic 则只会携带一段简短的默认信息
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    println!("{:?}", f);
}
