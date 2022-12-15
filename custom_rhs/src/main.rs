use std::ops::Add;

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
fn main() {
    let total = Millimeters(1000);
    println!("{:?}", total.add(Meters(1))); // Millimeters(2000)
}
