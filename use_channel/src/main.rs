// crossbeam-channel: https://crates.io/crates/crossbeam-channel
// 创建一个通道，每当接收到信号时，信号处理程序就向该通道发出一个值。
use crossbeam_channel::{bounded, select, tick, Receiver};
use std::{time::Duration};
use anyhow::{Result};
use ctrlc;

fn ctrl_channel() -> Result<Receiver<()>,ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}
fn main() -> Result<()> {
    let ctrl_c_events = ctrl_channel()?;
    let ticks = tick(Duration::from_secs(1));

    loop {
        // 监听 ctrl + c
        select! {
            recv(ticks) -> _ => {
                println!("working!");
            }
            recv(ctrl_c_events) -> _ => {
                println!("Goodbye!");
                break;
            }
        }
    }
    Ok(())
}


