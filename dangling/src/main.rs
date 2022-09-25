// 悬垂引用
fn main() {
    let reference_to_noting = dangle();
    println!("{}", reference_to_noting);
}

fn dangle() -> &String { // dangle 会返回一个指向 String 的引用
    let s = String::from("hello");

    &s // 将指向 s 的引用返回给调用者
} // 变量 s 在离开作用域并随之被销毁，它指向的内存自然也不再有效

// fix
// fn dangle() -> String {
//     let s = String::from("hello");

//     s
// }
