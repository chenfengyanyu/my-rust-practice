// 泛型生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x   
    } else {
        y
    }
}
// Test1：string1 直到外部作用域结束都会是有效的
// string2 的有效性则只持续到内部作用域结束的地方
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let res = longest(string1.as_str(), string2.as_str());
        println!("The longest string is: {}", res);
    }
}

// Test2：result 引用的生命周期必须要小于两个参数的生命周期
// error[E0597]: `string2` does not live long enough
//   --> src/main.rs:27:41
//    |
// 27 |         res = longest(string1.as_str(), string2.as_str());
//    |                                         ^^^^^^^^^^^^^^^^ borrowed value does not live long enough
// 28 |     }
//    |     - `string2` dropped here while still borrowed
// 29 |     println!("The longest string is: {}", res);
//    |                                           --- borrow later used here

// For more information about this error, try `rustc --explain E0597`.
// fn main() {
//     let string1 = String::from("long string");
//     let res;
//     {
//         let string2 = String::from("xyz");
//         res = longest(string1.as_str(), string2.as_str());
//         // 这里 string2 和 res 都有效，可以正常输出
//         println!("The longest string is: {}", res);
//     }
//     // 这里 string2 已经结束作用域范围，无效了，因此 res 不能比他的生命周期更久
//     // println!("The longest string is: {}", res);
// }
