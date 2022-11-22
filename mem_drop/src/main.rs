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
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main");
}

// drop(c) 输出：
// CustomSmartPointer created
// Dropping CustomSmartPointer with data `some data`!
// CustomSmartPointer dropped before the end of main

// 无 drop(c) 输出：
// CustomSmartPointer created
// CustomSmartPointer dropped before the end of main
// Dropping CustomSmartPointer with data `some data`!