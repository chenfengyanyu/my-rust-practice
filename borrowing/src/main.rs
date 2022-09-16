// fn main() {
//     let s1 = String::from("Hello");
//     // &s1 允许在不转移所有权的前提下，创建一个指向 s1 值的引用
//     // 由于引用不持有值的所有权，所以当引用离开当前作用域时，他指向的值也不会被丢弃
//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// // s 是一个指向 String 的引用
// fn calculate_length(s: &String)-> usize {
//     s.len()
// } // 这里，s 离开作用域，但不会销毁其指向的数据，因为它并不拥有该数据的所有权


// 可变引用
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("s value is: {}", s);
}

fn change(greet: &mut String){
    greet.push_str(", world");
    println!("greet value is: {}", greet)
    
}
