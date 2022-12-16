use std::ops::Add;

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);
// 指定 impl Add<Meters> 来设置 RHS 类型参数的值
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
fn main() {
    let total = Millimeters(1000);
    println!("{:?}", total.add(Meters(1))); // Millimeters(2000)
}

// 默认类型参数主要被用于以下两种场景
// 1.扩展一个类型而不破坏现有代码
// 2.允许在大部分用户都不需要的特定场合进行自定义
