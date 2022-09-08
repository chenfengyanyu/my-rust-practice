// 变量的可变性
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

// 隐藏机制，允许复用变量名称的同时改变他的类型
// fn main() {
//     let spaces = " ";
//     let spaces = spaces.len();
//     println!("spaces values is: {}", spaces);
// }

// 如果不改变类型，复用变量名会怎样？
// fn main() {
//     let spaces = " ";
//     let spaces = "1111";
//     println!("spaces values is: {}", spaces);
// }

// 可变类型不允许修改变量的类型
// fn main() {
//     let mut spaces = " ";
//     let spaces = spaces.len();
//     println!("now spaces values is: {}", spaces);
// }

