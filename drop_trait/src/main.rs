struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
fn main() {
    let c = CustomSmartPointer {
        data: String::from("first"),
    };
    let d = CustomSmartPointer {
        data: String::from("second"),
    };
    println!("CustomSmartPointer created.");
}

// 输出如下：
// CustomSmartPointer created.
// Dropping CustomSmartPointer with data `second`!
// Dropping CustomSmartPointer with data `first`!

// Rust 会自动调用我们在 drop 方法中放置的代码来打印出最终的信息，而无须显式地调用 drop 方法