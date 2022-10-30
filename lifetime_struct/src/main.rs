// 在结构体中存储引用，需要为结构体定义中的每一个引用都添加生命周期标注
struct ImportantExcerpt<'a> {
    part: &'a str
}
fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().expect("Could not find a '.'");
    let i = ImportantExcerpt{
        part: first_sentence
    };
    println!("novel: {}", i.part);
}
