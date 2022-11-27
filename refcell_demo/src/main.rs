use std::{rc::Rc, cell::RefCell};

#[derive(Debug)]
enum List {
    // 创建一个 Rc<RefCell<i32>> 实例，并将他暂时存入 value 变量中
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil
}

use crate::List::{Cons,Nil};
// use std::rc::Rc;
// use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));
    // 使用含有 value 的 Cons 变体创建一个 List 类型的 a 变量
    // 为了确保 a 和 value 同时持有内部值 5 的所有权，这里的代码还克隆了 value，而不仅仅只是将 value 的所有权传递给 a
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    // 为了让随后创建的 b 和 c 能够同时指向 a ，我们将 a 封装到 Rc<T> 中
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // 通过调用 borrow_mut 来将 value 指向的值增加 10 
    // 并使用自动解引用功能来将 Rc<T> 解引用为 RefCell<T>，borrow_mut 方法会返回一个 RefMut<T> 智能指针
    // 我们可以使用解引用运算符来修改其内部值
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// a after = Cons(RefCell { value: 15 }, Nil)
// b after = Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
// c after = Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
