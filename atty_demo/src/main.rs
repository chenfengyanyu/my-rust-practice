use atty::Stream;

fn main() {
  if atty::is(Stream::Stdout) {
    println!("I'm a terminal");
  } else {
    println!("I'm not");
  }
}