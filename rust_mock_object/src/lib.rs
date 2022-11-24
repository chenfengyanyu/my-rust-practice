pub trait Messenger {
    // 接收 self 的不可变引用及一条文本消息作为参数
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker { messenger, value: 0, max }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // 对 RefCell<T> 而言，我们需要使用 borrow 与 borrow_mut 方法来实现类似 & 与 &mut
    use std::cell::RefCell;
    // 使用动态数组来记录所有接收消息
    struct MockMessager {
        // 使用 RefCell<T> 来修改内部存储的值
        sent_message: RefCell<Vec<String>>
    }

    impl MockMessager {
        // 创建 MockMessager 实例
        fn new() -> MockMessager {
            MockMessager { sent_message: RefCell::new(vec![]) }
        }
    }
    // 为 MockMessager 实现了 Messenger trait，便于后续创建 LimitTracker
    impl Messenger for MockMessager {
        fn send(&self, msg: &str) {
            // borrow_mut 方法来获取 RefCell<Vec<String>>内部值（也就是动态数组）的可变引用
            self.sent_message.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messager = MockMessager::new();
        let mut limit_tracker = LimitTracker::new(&mock_messager, 100);

        limit_tracker.set_value(80);
        // 调用 borrow 方法来取得动态数组的不可变引用
        assert_eq!(mock_messager.sent_message.borrow().len(), 1);
    }
}
