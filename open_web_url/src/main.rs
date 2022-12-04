use webbrowser;

fn main() {
    if webbrowser::open("http://jartto.wang").is_ok() {
        println!("Open!");
    }
}
