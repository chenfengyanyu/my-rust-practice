// loop
// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {}", result);
// }

// while
// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{}!", number);

//         number = number - 1;
//     }

//     println!("LIFTOFF!!!");
// }

// for
fn main() {
    let a = [10, 20, 30, 40, 50];

    for ele in a.iter() {
        println!("The value is: {}", ele);
    }

    for num in (1..4).rev() {
        println!("{}!", num);
    }
    println!("LIFTOFF!!!");
}
