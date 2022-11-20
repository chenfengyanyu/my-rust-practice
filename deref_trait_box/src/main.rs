use std::ops::Deref;
// 定义 MyBox 为一个拥有 T 类型单元素的元组结构体
struct MyBox<T>(T);
impl <T> MyBox<T>{
    // 接收一个 T 类型的参数，并返回一个存储有传入值的 MyBox 实例作为结果
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl <T> Deref for MyBox<T> {
    // 定义 Deref trait 的关联类型 T
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // deref 会返回一个指向值得引用，进而允许调用者通过 * 运算符访问值
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
