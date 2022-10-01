#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 这个例子更好的理解借用和所有权
// 错误演示
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     println!("The area {}", area(rect1)); // 这样写 area 函数获得了 rect1 的所有权
//     println!("{:?}",rect1); // 这时候 rect1 已经被移除了，所以会报错
// }

// fn area(rectangle: Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// 正确演示
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area {}", area(&rect1)); // 这样写 area 函数借用结构体
    println!("{:?}",rect1); // main 函数依旧保留了所有权，并可以正常使用
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}



