use std::sync::{Mutex,Arc};
use std::thread;

fn main() {
    // Rc<T> 在跨线程使用时并不安全，这里需要用到 Arc<T> 可以安全地在多个线程间共享
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // 不能将 counter 的所有权移动到多个线程中，因此每次需要移动所有权至线程时克隆 Arc<T>
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    // 主线程中收集所有线程句柄，并通过逐一调用句柄的 join 方法来确保所有生成的线程执行完毕
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()); // Result: 10
}
