use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// trait Add<RHS=Self> { // RHS=Self 就是所谓的默认类型参数（default type parameter）
// 泛型参数 RHS （right-handle side)定义了 add 方法中 rhs 参数的类型
//     type Output;

//     fn add(self, rhs: RHS) -> Self::Output;
// }

impl Add for Point {
    // Add trait 拥有一个名为 Output 的关联类型，它被用来确定 add 方法的返回类型
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    println!("{:?}", Point { x: 1, y: 0 } + Point { x: 2, y: 3 }); // Point { x: 3, y: 3 }
}
