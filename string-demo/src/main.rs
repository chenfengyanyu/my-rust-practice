fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world!");

    let s = format!("{} {}", s1, s2);
    println!("{:?}", s);

    // 字符串切片
    let r = &s[..5];
    println!("{:?}", r);

    // 字符串遍历：chars
    for c in s.chars() {
        println!("{}", c);
    }

    // 遍历字节：bytes
    for b in s.bytes() {
        println!("{}", b);
    }
}
