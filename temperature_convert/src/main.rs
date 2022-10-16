use std::io;
// 实现摄氏温度与华氏温度相互转换
// 华式度 = 32 + 摄氏度 x 1.8; 摄氏度 = (华式度 - 32) / 1.8。（华式单位: °F， 摄氏单位: °C）
fn main() {
    loop {
        println!("请输入要转换的温度类型：摄氏温度【1】，华氏温度【2】");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取异常");

        let int_num: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入整数！");
                return;
            }
        };

        if int_num == 1 {
            println!("温度类型为：摄氏温度");
            let mut c_num = String::new();
            io::stdin().read_line(&mut c_num).expect("读取异常");
            
            let temp = c_num.parse::<f64>().unwrap();
            let trans_num = 32 as f64 + temp * 1.8;
            println!(">>>>{}",trans_num);
            // println!("摄氏温度：{} 对应的华氏温度为：{}", c_num, trans_num);
        } else if int_num == 2 {
            println!("温度类型为：华氏温度");
        } else {
            println!("温度类型异常，请重新设置！");
        }


    }

}


