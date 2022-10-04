use std::fs::File;
fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(err) => {
            panic!("There was a problem opening the file: {:?}", err);
        }
    };
    println!("{:?}", f);
}
