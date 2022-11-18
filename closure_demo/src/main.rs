use std::thread;
use std::time::Duration;

// T 代表一个使用 Fn trait 的闭包
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
// 运行闭包之前，value 会被初始化为 None
// 而当使用 Cacher 的代码请求闭包的执行结果时，Cacher 会运行闭包并将结果存储在 value 的 Some 变体中
// 之后，如果代码重复请求闭包的结果，Cacher 就可以避免再次运行闭包，而将缓存在 Some 变体中的结果返回给调用者
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                // 执行 self.calculation 闭包
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    // 方式一：通过变量存储了一个匿名函数定义，而不是调用该匿名函数而产生的返回值
    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    // 方式二：使用 Cacher 结构体
    // 将闭包存储在新创建的 Cacher 实例中
    let mut expensive_result = Cacher::new(|num|{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity))
        }
    }
}
