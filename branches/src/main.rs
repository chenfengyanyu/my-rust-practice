// fn main() {
//     let number = 3;

//     if number < 5 {
//         println!("condition was ture");
//     } else {
//         println!("condition was false");
//     }
// }

// 条件表达式需要同一个类型，否则将导致编译失败
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        "6" // ^^^ expected integer, found `&str`
    };

    println!("The value of number is: {}", number);
}