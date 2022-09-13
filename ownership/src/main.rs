fn main() {
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String){ // some_string 进入作用域
    println!("{}", some_string);
} // some_string 离开作用域，drop 函数被自动调用
// some_string 所占用的内存也就随之被释放了

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}
