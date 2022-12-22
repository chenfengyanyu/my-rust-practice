trait Animal {
    fn baby_name() -> String;
}

struct Dog;
impl Dog {
    fn bady_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
fn main() {
    // println!("A baby dog is called a {}",Dog::bady_name()); // A baby dog is called a Spot
    // 将 Dog 类型视作 Animal，并调用 Dog 为 Animal trait 实现的 baby_name 函数
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // A baby dog is called a puppy
}
