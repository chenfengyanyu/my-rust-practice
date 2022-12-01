use std::sync::Mutex;
fn main() {
    let m = Mutex::new(5);
    {
        // 调用 lock 方法获取锁，这个调用会阻塞当前线程直到我们取得锁为止
        // 当前线程对于 lock 函数的调用会在其他某个持有锁的线程发生 panic 时失败，通过 unwarp 在意外发生时触发当前线程的 panic
        let mut num = m.lock().unwrap(); // 一旦获取了锁，我们便可以将它的返回值 num 视作一个指向内部数据的可变引用
        *num = 6;
    }
    println!("m = {:?}", m); // m = Mutex { data: 6, poisoned: false, .. }
}
