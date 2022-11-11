use yew::{prelude::*, services::{fetch::{Request, Response, FetchTask}, FetchService}, format::Json}; // 预导入模块

struct TodoApp {
    link: ComponentLink<Self>,
    todos: Option<Vec<Todo>>, // todo list
    fetch_task: Option<FetchTask> // fetch data
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub user_id: u64,
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

enum Msg {
    MakeReq,
    Resp(Result<Vec<Todo>, anyhow::Error>),
}

impl Component for TodoApp {
    type Message = Msg;
    type Properties = ();
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::MakeReq);
        Self { link, todos: None, fetch_task: None }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeReq => {
                self.todos = None;
                let req = Request::get("https://jsonplaceholder.typicode.com/todos")
                .body(Nothing)
                .expect("can make req to jsonplaceholder");

                let cb = self.link.callback(|response: Response<Json<Result<Vec<Todo>, anyhow::Error>>>|{
                    let Json(data) = response.into_body();
                    Msg::Resp(data)
                });

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_task = Some(task);
                ()
            }
            Msg::Resp(resp) => {
                if let Ok(data) = resp {
                    self.todos = Some(data);
                }
            }
        }
        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html!{
            <div class=classes!("todo")>
                {"Hello"}
            </div>
        }
    }
}

fn main() {
    App::<TodoApp>::new().mount_to_body();
}
