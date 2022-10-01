mod back_of_house {
    #[derive(Debug)]
    #[allow(dead_code)] // 禁用未使用到的变量警告
    pub struct Breakfast {
        pub toast: String,
        fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("{:#?} toast please!", meal);
    // 结构体中 fruit 为私有字段，无法调用
    // meal.fruit = String::from("blueberries");
}

fn main() {
    eat_at_restaurant()
}
