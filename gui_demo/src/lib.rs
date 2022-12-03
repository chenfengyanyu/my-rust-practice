use gui::Draw;
pub trait Draw {
   fn draw(&self); 
}

pub struct  Screen<T: Draw> {
    pub components: Vec<T>
}

impl<T> Screen<T>
where T: Draw
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl  Draw for Button {
    fn draw(&self) {}
}

struct SelectBox {
    width: u32,
    height: u32,
    option: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        // 实际绘制一个选择框的代码
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    
}
