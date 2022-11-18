fn main() {
    let v1 = vec![1, 2, 3];

    // 迭代器是惰性的，创建迭代器后，除非主动调用，否则不会产生任何实际效果
    // 创建迭代器
    let v1_iter = v1.iter();
    // 使用 for 进行遍历
    for val in v1_iter {
        println!("Got: {}", val);
    }
}
