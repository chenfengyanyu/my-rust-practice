use std::thread;
fn main() {
    let v = vec![1,2,3];
    // 添加 move 关键字，强制闭包获得它所需值得所有权，而不仅仅是基于 Rust 的推导来获得值得借用
    // move 关键字覆盖了 Rust 的默认借用规则。但这并不意味着它会允许我们去违反任何的所有权规则
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v);

    handle.join().unwrap();
}

// Here's a vector: [1, 2, 3]