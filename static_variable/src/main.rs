static mut COUNTER: u32 = 0; // 使用 mut 来指定静态变量的可变性

fn add_to_count(inc: u32) {
    // 任何读写 COUNTER 的代码都必须位于 unsafe 代码块中
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    // 因为是单线程，所有当多个线程同时访问 COUNTER 时，则可能会出现数据竞争
    unsafe {
        println!("COUNTER: {}", COUNTER); // COUNTER: 3
    }
}
