# my-rust-practice
- 【基础】内容参考《Rust 权威指南》，作为练习记录。若代码有需要改进的地方还望不吝赐教，一起相互学习，共同进步！
- 【实践】来源于学习、工作以及其他辅助材料。
- 【扩展】补充一些 Rust 周边能力，如：镜像源、私有化部署等。
> 注意：如果不想保留 target 编译文件，可以使用命令一键清除：find . -name "target" | xargs rm -rf
## 一、基础
### 1.1 入门示例
- [x] 【2022.09.07】[first_demo](./first_demo/) Hello World
- [x] 【2022.09.07】[guessing_game](./guessing_game/) 猜数游戏

### 1.2 基本概念
- [x] 【2022.09.08】[variables](./variables/) 变量
- [x] 【2022.09.09】[compound_type](./compound_type/) 复合类型
- [x] 【2022.09.11】[loops](./loops/) 循环，[branches](./branches/) 条件判断，[try_function](./try_function/) 函数
- [x] 【2022.09.13】[ownership](./ownership/) 所有权
- [x] 【2022.09.16】[borrowing](./borrowing/) 借用
- [x] 【2022.09.25】[dangling](./dangling/) 悬垂引用
- [x] 【2022.09.26】[slice](./slice/) 切片
- [x] 【2022.09.27】[struct](./struct-demo/) 结构体
- [x] 【2022.09.29】[struct_demo2](./struct_demo2/) 结构体示例
- [x] 【2022.10.01】[struct_demo3](./struct_demo3/) impl实现结构体，[associated_func](./associated_fun/) 关联函数
- [x] 【2022.10.01】[enum_demo](./enum_demo/) enum 枚举类型，[option_demo](./option_demo/) option 枚举类型
- [x] 【2022.10.01】[match_demo](./match_demo/) match 控制流运算符

### 1.3 包、单元包及模块
- [x] 【2022.10.01】[lib_demo](./lib_demo/) 单元包
- [x] 【2022.10.01】[pub_struct](./pub_struct/) 定义公共结构体，[pub_enum](./pub_enum/) 定义公共枚举
- [x] 【2022.10.02】[use_crate](./use_crate/) use 导入单元包，[outside_lib](./outside_lib/) 导入自定义包文件

### 1.4 通用集合类型
- [x] 【2022.10.02】[vector_demo](./vector_demo/) 动态数组 Vector
- [x] 【2022.10.02】[string_demo](./string-demo/) 字符串 String
- [x] 【2022.10.02】[hashmap_demo](./hashmap_demo/) 哈希映射 HashMap，[hashmap_demo2](./hashmap_demo2/)示例：单词查找
- [x] 【2022.10.30】[basic_next](./basic_next/) next() 基本用法

### 1.5 错误处理
- [x] 【2022.10.04】[panic_demo](./panic_demo/) 使用 panic
- [x] 【2022.10.04】[unwarp_expect](./unwrap_expect/) unwarp 和 expect
- [x] 【2022.10.04】[error_back](./error_back/) ？运算符返回错误

### 1.6 泛型、trait与生命周期
- [x] 【2022.10.05】[generics](./generics_demo/) 泛型示例：求最大数
- [x] 【2022.10.05】[struct_generics](./struct_generics/) 结构体中使用泛型
- [x] 【2022.10.27】[trait_demo](./trait_demo/) Trait 的基本使用：定义共同行为
- [x] 【2022.10.27】[trait_demo_default](./trait_demo_default/) Trait 的基本使用：默认实现
- [x] 【2022.10.28】[trait_demo_type](./trait_deom_type/) Trait 的基本使用：Trait 作为参数
- [x] 【2022.10.28】[trait_demo_largest](./trait_demo_largest/) Trait 的基本使用：Trait 实现 largest 函数
- [x] 【2022.10.29】[trait_demo_if](./trait_demo_if/) Trait 的基本使用：使用 trait 约束来有条件地实现方法
- [x] 【2022.10.29】[lifetime_if](./lifetime_if/) 生命周期：miss lifetime specifier
- [x] 【2022.10.29】[lifetime_test](./lifetime_test/) 生命周期：处理不同生命周期的引用
- [x] 【2022.10.30】[lifetime_struct](./lifetime_struct/) 生命周期：结构体中定义
- [x] 【2022.10.30】[lifetime_trait](./lifetiem_trait/) 生命周期：同时使用泛型、trait约束与生命周期

### 1.7 闭包 Closure
- [x] 【2022.11.17】[closure_demo](./closure_demo/) 闭包：能够捕获环境的匿名函数
- [x] 【2022.11.19】[closure_filter](./closure_filter/) 使用闭包捕获环境

### 1.8 迭代器 Iterator
- [x] 【2022.11.18】[iter_demo](./iter_demo/) 创建一个迭代器
- [x] 【2022.11.18】[iter_trait_demo](./iter_trait_demo/) Iterator trait 和 next 方法
- [x] 【2022.11.19】[custom_iter](./custom_iterator/) 创建自定义迭代器

### 1.9 智能指针
- [x] 【2022.11.20】[list_box](./list_box/) 使用 Box<T> 的 List 定义
- [x] 【2022.11.20】[deref_trait_box](./deref_trait_box/) 通过实现 Deref trait 来将类型视作引用
- [x] 【2022.11.21】[drop_trait](./drop_trait/) 借助 Drop trait 在清理时运行代码
- [x] 【2022.11.22】[mem_drop](./mem_drop/) 使用 std::mem::drop 提前丢弃值
- [x] 【2022.11.22】[rc_list](./rc_list/) 基于引用计数的智能指针 Rc<T>，Reference counting（引用计数）
- [x] 【2022.11.23】[rc_strong_count](./rc_strong_count/) 克隆 Rc<T> 会增加引用计数
- [x] 【2022.11.24】[rust_mock_object](./rust_mock_object/) 模拟对象
- [x] 【2022.11.27】[refcell_demo](./refcell_demo/) 将 Rc<T> 和 RefCell<T> 结合使用来实现一个拥有多重所有权的可变数据

### 1.10 无畏并发
- [x] 【2022.11.27】[handle_thread](./handle_thread/) 使用 join 句柄等待所有线程结束
- [x] 【2022.11.27】[move_thread](./move_thread/) 在线程中使用 move 闭包
- [x] 【2022.11.27】[message_passing](./message_passing/) 使用消息传递在线程间转移数据
- [x] 【2022.12.01】[mutual_exclusion](./mutual_exclusion/) 互斥体：mutex（mutual exclusion）
- [x] 【2022.12.01】[mutex<T>](./mutex/) 在多个线程间共享 Mutex<T>

### 1.11 面向对象编程特性
- [x] 【2022.12.02】[encapsulation](./encapsulation/) 封装示例
- [x] 【2022.12.03】[gui_demo](./gui_demo/) 使用 trait 对象来存储不同类型的值
- [x] 【2022.12.03】[blog_demo](./blog_demo/) 状态模式示例（实现一个发布博客的工作流）

### 1.12 unsafe 特性
- [x] 【2022.12.11】[unsafe_demo](./unsafe_demo/) 通过不安全函数来实现 split_at_mut
- [x] 【2022.12.14】[extern_func](./extern_func/) 用 extern 关键字来简化创建和使用外部函数接口(Foreign Function Interface，FFI)
- [x] 【2022.12.14】[static_variable](./static_variable/) 声明、访问与修改可变静态变量（Rust 中全局变量)
- [x] 【2022.12.14】[unsafe_trait](./unsafe_trait/) 实现不安全 trait

### 1.13 高级特性
- [x] 【2022.12.15】[operator_overloading](./operator_overloading/) 通过实现 Add trait 来重载 Point 实例的+运算符
- [x] 【2022.12.15】[custom_rhs](./custom_rhs/) 设置自定义 RHS（right-handle side）类型参数
- [x] 【2022.12.16】[same_trait_fn](./same_trait_fn/) 指定调用某个 Trait 上的方法
- [x] 【2022.12.22】[fully_qualified_syntax](./fully_qualified_syntax/) 使用完全限定语法 <Type as Trait>


## 二、实践
### 2.1 小练习
- [x] 【2022.10.12】[fibonacci_sequence](./fibonacci_sequence/) 斐波那契数列
- [x] 【2022.10.12】[temperature_convert](./temperature_convert/) 摄氏温度与华氏温度的相互转换1
- [x] 【2022.10.18】[temperature_convert_2](./temperature_convert_2/) 摄氏温度与华氏温度的相互转换2
- [ ] 【2022.10.13】[打印歌词并循环处理重复内容](#) 打印歌词并循环处理重复内容

### 2.2 编写一个游戏
- [x] 【2022.10.16】[my_box_game](./my_box_game/) 示例游戏
![示例](./my_box_game/demo/2022-10-16%2012.20.43.gif)

### 2.3 编写 CLI 工具
- [x] 【2022.10.20】[rust_cli_demo](https://github.com/chenfengyanyu/rust_cli_demo/tree/main) Rust CLI Demo —— Hello
- [x] 【2022.10.21】[clap_cli_demo](./clap_cli_demo/) 使用 CLI 实现 cat 命令
- [x] 【2022.11.01】[rust_minigrep](https://github.com/chenfengyanyu/rust_minigrep) 使用 CLI 实现 grep 命令
- [x] 【2022.11.20】[show_rust_cli](https://github.com/chenfengyanyu/show_rust_cli) Command Line Applications in Rust

### 2.4 测试相关
- [x] 【2022.10.25】[criterion_bench_demo](./criterion_bench_demo/) Rust 性能测试（起手测试）
- [x] 【2022.10.30】[try_test](./try_test/) 编写单元测试
- [x] 【2022.10.31】[try_test_panic](./try_test_panic/) 使用 should_panic 检查 panic
- [x] 【2022.10.31】[test_result](./test_result/) 使用 Result<T, E>编写测试
- [x] 【2022.10.31】[try_adder_tests](./try_adder_tests/) 编写集成测试

### 2.5 框架使用：Yew 
> Yew 是一个设计先进的 Rust 框架，目的是使用 WebAssembly 来创建多线程的前端 Web 应用。
- [x] 【2022.10.25】[yew_app](./yew_app/) Rust Web 框架：Yew + Bulma
- [x] 【2022.11.05】[rust-web](https://github.com/chenfengyanyu/rust-web) Rust Web 全栈开发（RESTful API，WebAssembly）
- [x] 【2022.11.13】[yew_todolist](./yew_todolist/) Yew Todolist（use trunk serve）

### 2.6 捕获快捷键
- [x] 【2022.11.22】[signal_handling](./signal_handling/) 处理快捷键 Ctrl+C
- [x] 【2022.11.22】[ctrlc_demo](./ctrlc_demo/) 使用三方库 ctrlc 捕获键位
- [x] 【2022.11.26】[handle_other_signals](./handle_signals/) 捕获其他类型的信号(Crates: Signal-hook)
- [x] 【2022.11.26】[use_channel](./use_channel/) 使用 channel 通道进行信号处理(Crates: crossbeam-channel)

### 2.7 系统信息
- [x] 【2022.12.04】[system_info](./get_sys_info/) 获取系统信息
- [x] 【2022.12.04】[system_info2](./sysinfo/) Get system information in Rust
- [x] 【2022.12.04】[open_web_browsers](./open_web_url/) 在浏览器中打开 URL


### 2.8 其他
- [x] 【2022.11.26】[confy_demo](./confy_demo/) 使用[confy](https://crates.io/crates/confy)处理配置文件
- [x] 【2022.11.26】[exit_code](./exit_code/) 使用[exitcode](https://crates.io/crates/exitcode)设置退出码
- [x] 【2022.11.26】[human_panic](./human_panic/) [human-panic](https://crates.io/crates/human-panic): Panic messages for humans
- [x] 【2022.11.26】[atty_demo](./atty_demo/) 使用[atty](https://crates.io/crates/atty)

## 三、扩展
### 3.1 Cargo 镜像源
- [x] 【2022.11.03】[crates_image](./crates_images/) crates.io 镜像

### 3.2 Rust Crates 私有化部署
- [x] 【2022.11.03】[private_registry](./private_registry/) 部署指南

### 3.3 Rust 文档注释
- [x] 【2022.11.19】[rust_docs](./rust_docs/) 编写文档示例
![docs demo](https://github.com/chenfengyanyu/my-rust-practice/blob/main/rust_docs/src/image/show.png?raw=true)


## 四、学习资源
- [x] 【2022.11.28】[rust_source](./rust_source/) Rust 入门与练习


