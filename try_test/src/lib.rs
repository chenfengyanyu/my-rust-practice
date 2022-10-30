// 通过 cargo new xxx --lib 创建
use rand;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test] // 标记当前函数为测试
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test] // 一旦测试函数触发 panic，该测试就被视作执行失败
    fn another() {
        // 增加单测随机性
        let rng = rand::thread_rng().gen_range(0, 11);
        if rng % 2 == 0 {
            println!("Everything is ok!");
        } else {
            panic!("Something is wrong!");
        }
    }

    #[test]
    fn largest_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            length: 9,
        };
        let smaller = Rectangle {
            length: 2,
            width: 3,
        };

        assert!(larger.can_hold(&smaller));
    }
}
