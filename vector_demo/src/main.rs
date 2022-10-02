fn main() {
    let mut v = Vec::new();
    v.push(1);
    println!("{:?}", v);

    // vec! 宏
    let mut vs = vec![1,3,5,7];
    let third = &vs[2];
    println!("{}", third);

    // 遍历动态数组
    for i in &mut vs {
        // 使用解引用运算符 * 来获得 i 绑定的值
        *i += 50;
        println!("{:?}", i);
    }
}
