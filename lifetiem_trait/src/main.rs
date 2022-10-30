use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x   
    } else {
        y
    }
}
fn main() {
    let res = longest_with_an_announcement("hello", "abc", 666);
    println!("Result: {}", res);
}
