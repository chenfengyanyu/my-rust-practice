fn main() {
    println!("Hello, world!");

    another_function(5);
}

fn another_function(x: i32) {
    println!("Another function value is {}", x);
    statement_expression();
}

fn statement_expression() {
    let y = 5;

    let z = {
        let y = 3;
        y + 1
    };

    println!("The value of y is: {}", z);
}
