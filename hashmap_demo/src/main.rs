use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let team_name = String::from("Blue");
    // get 返回一个 Option<&V> ，所有这里的结果被封装到了 Some 中
    let score = scores.get(&team_name);
    println!("{:?}", score);

    // 遍历哈希映射
    for (key, val) in &scores {
        println!("{}: {}", key, val);
    }

    // 更新
    scores.insert(String::from("Blue"), 15);
    println!("{:?}", scores);
    // Entry 枚举，or_insert 方法检测值是否存在
    scores.entry(String::from("Blue")).or_insert(66);
    scores.entry(String::from("Red")).or_insert(88);
    println!("{:?}", scores);
}
