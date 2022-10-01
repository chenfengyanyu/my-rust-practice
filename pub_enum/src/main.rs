mod back_of_house {
    // #[warn(dead_code)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let _order1 = back_of_house::Appetizer::Soup;
}

fn main() {
    println!("Hello, world!");
}
