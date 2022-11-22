enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use crate::List::{Cons, Nil};
// 将 Rc 引入作用域
use std::rc::Rc;
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // 创建 b 时调用的 Rc::clone 函数会会接收 a 中 Rc<List> 的引用作为参数
    // Rc::clone 不会执行数据的深度拷贝操作，只会增加引用计数，且不会花费太多时间
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
