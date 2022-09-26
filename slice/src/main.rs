fn main() {
    let mut s = String::from("hello world");

    // 正常读取
    let hello =  first_word(&s);
    println!("Normal: {}", hello);
    
    // 字符串切片
    let word = &s[..5];
    println!("Slice: {}", word);
    
    println!("s: {}", s);
    // 清空当前字符串，使之变为“”
    s.clear();
}

fn first_word(s: &String)  -> &str {
    // 将 String 转换为字节数组
    let bytes = s.as_bytes();

    // 创建可以遍历的字节数组迭代器，iter 方法会依此返回集合中的每一个元素
    for(i, &item) in bytes.iter().enumerate() { // enumerate 将 iter的每个输出作为元素逐一封装在对应的元组中返回
        if item == b' ' {
            return  &s[0..i];
        }
    }

    &s[..]
}
