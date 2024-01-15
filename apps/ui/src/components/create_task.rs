use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::services::tasks_service::create_task;

pub struct CreateTaskComponent {
    pub title: String,
    pub task_created: Callback<String>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub task_created: Callback<String>,
}

pub enum Msg {
    UpdateTitle(String),
    Add,
}

impl Component for CreateTaskComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            title: "".to_string(),
            task_created: ctx.props().task_created.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateTitle(title) => {
                self.title = title.clone();
                web_sys::console::log_1(&title.into());
            }
            Msg::Add => {
                let title = self.title.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    create_task(title).await.unwrap();
                });

                self.task_created.emit(self.title.clone());
                self.title.clear();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="create-task">
                <input type="text" id="task" placeholder="Enter a new task" value={self.title.clone()}
                onchange={ctx.link().callback(|e: Event| {
                    let input = e.target_dyn_into::<HtmlInputElement>();
                    Msg::UpdateTitle(input.unwrap().value())
                })}/>
                <button onclick={ctx.link().callback(|_| Msg::Add)}>{ "Add" }</button>
            </div>
        }
    }
}
