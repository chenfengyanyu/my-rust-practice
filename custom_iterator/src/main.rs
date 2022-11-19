struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    // 指定迭代器的关联类型为 u32
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 2;

        if self.count < 8 {
            Some(self.count)
        } else {
            None
        }
    }
}
fn main() {
    let mut counter = Counter::new();

    println!("{:?}", counter.next()); // Some(2)
    println!("{:?}", counter.next()); // Some(4)
    println!("{:?}", counter.next()); // Some(6)
    println!("{:?}", counter.next()); // None
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        // zip 可以将两个 iter 合并成一组 iter，它在两个迭代器中的任意一个返回 None 时结束迭代
        .zip(Counter::new().skip(1))
        // 对上面的一组 iter 执行乘积 2*4, 4*6
        .map(|(a, b)| a * b)
        // filter 之后只有 24
        .filter(|x| x % 3 == 0)
        // 求和
        .sum();
    assert_eq!(24, sum);
}
