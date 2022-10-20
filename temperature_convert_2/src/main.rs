// 参考：https://blog.csdn.net/qq_36393978/article/details/125805264
use std::io;

enum Type {
    Fahrenheit,
    Celsius,
    None,
}

struct Temp {
    number: f64,
    temp_type: Type,
}

fn get_temp() -> Temp {
    let mut input = String::new();
    let mut temp_input: Temp = Temp {
        number: 0.0,
        temp_type: Type::None,
    };

    println!("Please enter a key(f is Fahrenheit, c is Celsius):");
    io::stdin().read_line(&mut input).expect("读取异常");

    match input {
        mut input_01 if (input_01.trim() == "F") || (input_01.trim() == "f") => loop {
            println!("Enter Fahrenheit:");
            input_01.clear();

            io::stdin().read_line(&mut input_01).expect("读取异常");
            temp_input.temp_type = Type::Fahrenheit;
            temp_input.number = match input_01.trim().parse::<f64>() {
                Ok(temp) => temp,
                Err(_) => {
                    println!("输入数字");
                    continue;
                }
            };
            break;
        },
        mut input_02 if (input_02.trim() == "C") || (input_02.trim() == "c") => loop {
            println!("Enter Celsius:");
            input_02.clear();

            io::stdin().read_line(&mut input_02).expect("读取异常");
            temp_input.temp_type = Type::Fahrenheit;
            temp_input.number = match input_02.trim().parse::<f64>() {
                Ok(temp) => temp,
                Err(_) => {
                    println!("输入数字");
                    continue;
                }
            };
            break;
        },
        _ => println!("异常输入"),
    }
    temp_input
}

fn print_temp(temp: Temp) {
    println!(
        "Current temp: {} {}",
        temp.number,
        match temp.temp_type {
            Type::Fahrenheit => "°F",
            Type::Celsius => "°C",
            _ => "Error",
        }
    )
}

fn convert_temp(mut temp: Temp) {
    match temp.temp_type {
        Type::Fahrenheit => {
            temp.number = (temp.number - 32.0) / 1.8;
            temp.temp_type = Type::Celsius;
        },
        Type::Celsius => {
            temp.number = 32.0 + temp.number * 1.8;
            temp.temp_type = Type::Celsius;
        },
        _ => println!("Error")
    }
    print_temp(temp)
}

fn main() {
    convert_temp(get_temp());
}
