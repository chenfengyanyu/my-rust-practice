use std::io;
fn main() {
    println!("打印斐波那契数列");
    println!("请输入数列长度：");
    let mut len = String::new();
    io::stdin().read_line(&mut len).expect("读取异常");

    let mut i: i32 = match len.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("请输入整数！");
        }
    };

    let mut arr = vec![];
    while i > 0 {
        arr.push(fibonacci_sequence(i));
        i -= 1;
    }
    arr.sort();

    println!("Fibonacci Sequence is: {:?}", arr);
}
// 斐波那契数列：1,1,2,3,5,8,13,21...
// 递归
fn fibonacci_sequence(n: i32) -> i32 {
    if n <= 0 || n > 30 {
        panic!("数列长度异常")
    }
    if n <= 2 {
        1
    } else {
        fibonacci_sequence(n - 1) + fibonacci_sequence(n - 2)
    }
}

// 循环版本
// fn fibonacci_sequence(mut n: i32) -> i32 {
//     if n <= 0 {
//         panic!("数列长度异常");
//     }

//     let mut p1 = 1;
//     let mut p2 = 0;
//     let mut sum = 1;
//     while n > 0 {
//         sum = p1 + p2;
//         p1 = p2;
//         p2 = sum;
//         n -= 1;
//     }
//     sum
// }
