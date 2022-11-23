use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // Rc::strong_count（强引用计数）用来读取引用计数，
    println!("count after creating a = {}", Rc::strong_count(&a));
    
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let _c = Cons(4, Rc::clone(&a));
        // Rc<T> 的 Drop 实现会在离开作用域时自动将引用计数减一
        println!("count after creating c = {}", Rc::strong_count(&a)); // c 离开作用域杯丢弃，引用计数减少1
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// 输出
// count after creating a = 1
// count after creating b = 2
// count after creating c = 3
// count after c goes out of scope = 2
