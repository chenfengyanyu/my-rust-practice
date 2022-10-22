extern crate clap;
use clap::{App, Arg};
// https://doc.rust-lang.org/std/path/struct.Path.html#method.exists
use std::path::Path;
// https://doc.rust-lang.org/std/process/index.html
use std::process;
// https://doc.rust-lang.org/std/fs/index.html
use std::fs::File;
use std::io::{Read, Write};

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
            match File::open(file) {
                Ok(mut f) => {
                    let mut data = String::new();
                    f.read_to_string(&mut data)
                        .expect("[kt Error] Unable to read the  file.");
                    // 获取全局 stdout 对象
                    let stdout = std::io::stdout();
                    // 可选项：将 handle 包装在缓冲区中
                    let mut handle = std::io::BufWriter::new(stdout);
                    match writeln!(handle, "{}", data) {
                        Ok(_res) => {}
                        Err(err) => {
                            eprintln!("[kt Error] Unable to display the file contents. {:?}", err);
                            process::exit(1);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("[kt Error] Unable to read the file. {:?}", err);
                    process::exit(1);
                }
            }
        } else {
            eprintln!("[kt Error] No such file or directory.");
            // 程序错误终止时的标准退出码
            process::exit(1);
        }
    }
}

// 运行：cargo run -- ./src/test.txt
// 构建：cargo build --release
// 安装：cargo install --path .
// 查看：clap_cli_demo -V
// 使用：clap_cli_demo -- ./src/test.txt
