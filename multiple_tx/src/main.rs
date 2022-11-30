use std::{sync::mpsc, thread, time::Duration}; // multiple producer, single comsumer

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];

        for val in vals {
            tx.send(val).unwrap();
            // 每条消息间隔1秒
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 将 rx 视作迭代器，不显式的调用 recv 函数
    // 迭代中的代码会打印出每个接收到的值，并在通道关闭时退出循环
    for received in rx {
        println!("Got: {}", received);
    }
}

// 注意：上述代码并没有在主线程的 for 循环中执行暂停或延迟指令，这也就表明主线程确实是在等待接收新线程中传递过来的值
// Got: hi
// Got: from
// Got: the
// Got: thread