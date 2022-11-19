#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String,
}

impl Shoe {
    fn shoes_in_my_siez(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter()   // 调用 into_iter 来创建可以获取动态数组所有权的迭代器
        .filter(|s| s.size == shoe_size) // 通过 filter 来将这个迭代器适配成一个新的迭代器
        .collect() // 消费迭代器
    }
}

fn main() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = Shoe::shoes_in_my_siez(shoes, 10);
    println!("{:#?}", in_my_size);
}
