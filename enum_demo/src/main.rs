#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}
fn main() {
    let home = IpAddr::V4("127.0.0.1".to_string());
    let loopback = IpAddr::V6("::1".to_string());
    println!("{:#?},{:#?}", home, loopback);
}
