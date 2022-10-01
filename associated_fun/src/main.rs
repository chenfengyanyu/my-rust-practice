#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 关联函数通常被用作构造器来返回一个结构体的新实例
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size * 2,
        }
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The Area {}", rect1.area());
    // 类型后添加 :: 来调用关联函数
    let sq = Rectangle::square(3);
    println!("{:#?}", sq);
}
