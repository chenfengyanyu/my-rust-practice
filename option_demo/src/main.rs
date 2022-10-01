#[derive(Debug)]
enum Option<T> {
    One(T),
    None,
}

fn main() {
    let num = Option::One(5);
    let str = Option::One("a string");

    let absent_num: Option<i32> = Option::None;
    println!("num: {:?}, str: {:?}, absent_num: {:?}", num, str, absent_num);
}
