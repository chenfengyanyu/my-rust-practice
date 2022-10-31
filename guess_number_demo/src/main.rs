use rand::prelude::*;
use std::io;

fn main() {
    let sys_num = rand::thread_rng().gen_range(0..10);
    println!("随机数：{}", sys_num);

    loop {
        println!("请输入一个数字：");
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("读取异常");
        // println!("输入数字为：{}", num);

        let num: i32 = num.trim().parse().unwrap();

        if num > sys_num {
            println!("Too big!");
        } else if num < sys_num {
            println!("Too small!");
        } else {
            println!("You win!");
            break;
        }
    }
}
