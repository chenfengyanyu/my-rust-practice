// 参考：https://blog.csdn.net/qq_36393978/article/details/125805264
use std::io;

enum Type {
    Fahrenheit,
    Celsius,
    None,
}

struct Temp {
    number: f64,
    temp_type: Type,
}

fn get_temp() -> Temp {
    let mut input = String::new();
    let mut temp_input: Temp = Temp {
        number: 0.0,
        temp_type: Type::None,
    };
}

fn main() {
    println!("Hello, world!");
}
