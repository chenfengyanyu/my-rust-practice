fn main() {
    let v1 = vec![1, 2, 3];

    // 这里 v1_iter 必须是可变的，因为调用 next 方法改变了迭代器内部用来记录序列位置的状态
    let mut v1_iter = v1.iter();
    // 可以理解为：每次调用 next 都吃掉迭代器中的一个元素
    println!("{:?}", v1_iter.next()); // Some(1)
    println!("{:?}", v1_iter.next()); // Some(2)
    println!("{:?}", v1_iter.next()); // Some(3)
    println!("{:?}", v1_iter.next()); // None
    println!("{:?}", v1_iter.next()); // None
}
