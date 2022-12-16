trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("From pilot fly");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("From wizard fly");
    }
}

impl Human {
    fn fly(&self) {
        println!("From human fly");
    }
}
fn main() {
    let person = Human;
    // 显式指定要调用的 trait 上的方法
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
