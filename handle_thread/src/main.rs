use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // 如果放置在这里，由于主线程会等待新线程执行完毕后才开始执行自己的 for 循环，所以他的输出将不再出现交替的情形
    handle.join().unwrap();
    // hi number 1 from the spawned thread!
    // hi number 2 from the spawned thread!
    // hi number 3 from the spawned thread!
    // hi number 4 from the spawned thread!
    // hi number 5 from the spawned thread!
    // hi number 6 from the spawned thread!
    // hi number 7 from the spawned thread!
    // hi number 8 from the spawned thread!
    // hi number 9 from the spawned thread!
    // hi number 1 from the main thread!
    // hi number 2 from the main thread!
    // hi number 3 from the main thread!
    // hi number 4 from the main thread!

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // 等待关联的线程完成
    // handle.join().unwrap();
}

// hi number 1 from the main thread!
// hi number 1 from the spawned thread!
// hi number 2 from the spawned thread!
// hi number 2 from the main thread!
// hi number 3 from the spawned thread!
// hi number 3 from the main thread!
// hi number 4 from the spawned thread!
// hi number 4 from the main thread!
// hi number 5 from the spawned thread!
// hi number 6 from the spawned thread!
// hi number 7 from the spawned thread!
// hi number 8 from the spawned thread!
// hi number 9 from the spawned thread!
