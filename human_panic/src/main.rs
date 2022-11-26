use human_panic::setup_panic;
// 添加一个自定义的 panic 处理程序是一个很好的想法，它提供了更多的以终端用户为中心的输出
fn main() {
    setup_panic!();
    println!("A normal log message");
    panic!("OMG EVERYTHING IS ON FIRE!!!")
}
