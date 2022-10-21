extern crate clap;
use clap::{App, Arg};
// https://doc.rust-lang.org/std/path/struct.Path.html#method.exists
use std::path::Path;
// https://doc.rust-lang.org/std/process/index.html
use std::process;
// https://doc.rust-lang.org/std/fs/index.html
use std::fs::File;
use std::io::Read;

fn main() {
    let matches = App::new("kt")
        .version("0.0.1")
        .author("jartto")
        .about("Try cat use clap")
        .arg(
            Arg::with_name("FILE")
                .help("File to print.")
                .empty_values(false),
        )
        .get_matches();

    if let Some(file) = matches.value_of("FILE") {
        println!("Value for file argument: {}", file);
        // 检查文件是否存在
        if Path::new(&file).exists() {
            println!("You get it!!!");
            let mut f = File::open(&file).expect("File not found.");
            let mut data = String::new();
            f.read_to_string(&mut data).expect("Unable to read the  file.");
            println!("{}", data);
        } else {
            eprintln!("[kt Error] No such file or directory.");
            // 程序错误终止时的标准退出码
            process::exit(1);
        }
    }
}
