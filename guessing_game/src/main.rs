// 引入标准库输入/输出
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_numer = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_numer);

    println!("Please input your guess.");

    // 声明可变变量，并绑定一个新的空白字符串
    let mut guess = String::new();

    // 处理终端中的标准输入
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}