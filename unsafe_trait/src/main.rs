unsafe trait Foo {
    // 某些方法
}
unsafe impl Foo for i32 {
    // 对应的方法实现
}
fn main() {
    println!("Hello, world!");
}

//  你可以在拥有充足理由时使用 unsafe，并在出现问题时通过显示标记的 unsafe 关键字来较为轻松的定位到它们