use std::sync::mpsc; // multiple producer, single consumer
use std::thread;
fn main() {
    // 函数 mpsc::channel 会返回一个含有发送端与接收端的元组
    let (tx, rx) = mpsc::channel();
    // 生成一个新线程，新线程必须拥有通道发送端的所有权才能通过通道来发送消息
    // 使用 move 关键字将 tx 移动到了闭包的环境中
    thread::spawn(move || {
        let val = String::from("hi");
        // 通过 send 方法来接收我们想要发送的值，并返回 Result<T, E> 类型的值作为结果
        // 当接收端已经被丢弃而无法继续传递内容时，执行发送操作会返回一个错误
        tx.send(val).unwrap();
    });

    // 使用 recv 会阻塞主线程的执行直到有值被传入通道
    // 一旦有值被传入通道，recv 就会将它包裹在 Result<T, E> 中返回
    // 如果通道的发送端全部关闭了，recv 则会返回一个错误来表明当前通道再也没有可接收的值
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// 输出：Got: hi
// try_recv 方法不会阻塞线程，它会立即返回 Result<T, E>;