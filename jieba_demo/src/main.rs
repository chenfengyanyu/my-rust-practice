// https://github.com/messense/jieba-rs
use jieba_rs::Jieba;

fn main() {
    let jieba = Jieba::new();
    let words = jieba.cut("我们中出了一个叛徒", false);
    println!("分词结果：{:?}", words); // 分词结果：["我们", "中", "出", "了", "一个", "叛徒"]
}
