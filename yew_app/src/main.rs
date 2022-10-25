use yew::prelude::*;
enum Msg {
    AddOne
}
struct Model {
    // `ComponentLink` is like a reference to a component
    // It can be used to send messages to the component
    link: ComponentLink<Self>, 
    value: i64
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        self {
            link,
            value: 0
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        
    }
}

fn main() {
    println!("Hello, world!");
}
