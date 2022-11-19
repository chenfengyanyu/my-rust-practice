// fn main() {
//     let v1 = vec![1, 2, 3];

//     // 这里 v1_iter 必须是可变的，因为调用 next 方法改变了迭代器内部用来记录序列位置的状态
//     let mut v1_iter = v1.iter();
//     // 可以理解为：每次调用 next 都吃掉迭代器中的一个元素
//     println!("{:?}", v1_iter.next()); // Some(1)
//     println!("{:?}", v1_iter.next()); // Some(2)
//     println!("{:?}", v1_iter.next()); // Some(3)
//     println!("{:?}", v1_iter.next()); // None
//     println!("{:?}", v1_iter.next()); // None
// }


// fn main() {
//     let v1 = vec![1, 2, 3];
//     let v1_iter = v1.iter();
//     // 调用 sum 方法来得到迭代器中的所有元素的总和
//     // 注意：调用 sum 过程中获取了迭代器 v1_iter 的所有权，所以该迭代器无法继续被随后的代码使用
//     let total: i32 = v1_iter.sum();
//     println!("{}", total);
// }


fn main() {
    let v1 = vec![1,2,3];
    // 迭代器适配器是惰性的，除非我们消耗迭代器，否则什么事情都不会发生
    // 消耗迭代器可以使用 collect 方法，它将结果值收集到某种集合数据类型中
    let v2: Vec<i32> = v1.iter().map(|x| x+1).collect();
    println!("{:?}", v2);
}