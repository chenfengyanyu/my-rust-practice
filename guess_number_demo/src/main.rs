use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret = rand::thread_rng().gen_range(0..101);
    // println!("神秘数字是：{}", secret);
    loop {
        println!("请猜一个数：");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取异常");
        println!("您猜的数字是：{}", guess);

        let guess: i32 = guess.trim().parse().unwrap();

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
