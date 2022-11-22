use std::{thread, time::Duration};
use ctrlc;

fn main() {
    // 使用三方库 Ctrlc：https://crates.io/crates/ctrlc
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
    }).expect("Error setting Ctrl-C handler");

    // 增加一个等待时长，确保可以进行 Ctrl+C 的用户测试
    thread::sleep(Duration::from_secs(2));
}
