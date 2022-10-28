fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
fn main() {
    let num_list = vec![10, 20, 30, 40, 50];

    let result = largest(&num_list);
    println!("The largest number is {}", result);

    let char_list = vec!['j', 'a', 'r', 't', 't', 'o'];

    let result2 = largest(&char_list);
    println!("The largest char is {}", result2);
}
