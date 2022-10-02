mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Hello, world!");
        }
    }
}
// pub use 使一个名称可以在新作用域中被其他任意代码使用
pub use crate::front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();
}
