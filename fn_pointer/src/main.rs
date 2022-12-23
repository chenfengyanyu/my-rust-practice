fn add_one(x: i32) -> i32 {
    x + 1
}
// fn 类型也就是所谓的函数指针（function pointer），将参数声明为函数指针时使用的语法与闭包类似
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer); // The answer is: 12
}
