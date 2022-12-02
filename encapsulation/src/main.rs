#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }
    pub fn average(&self) -> f64 {
        self.average
    }
    pub fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
fn main() {
    let mut num = AveragedCollection {
        list: vec![1, 2, 3],
        average: 0.0,
    };

    println!("Value is: {:?}", num);
    num.add(4);
    println!("Value is: {:?}", num);
}

// Value is: AveragedCollection { list: [1, 2, 3], average: 0.0 }
// Value is: AveragedCollection { list: [1, 2, 3, 4], average: 2.5 }
