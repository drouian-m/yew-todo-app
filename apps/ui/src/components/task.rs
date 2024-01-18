use types::Task;
use yew::prelude::*;

pub struct TaskComponent {
    data: Task,
}

pub enum Msg {
    Toggle(),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub task: Task,
}

impl Component for TaskComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            data: ctx.props().task.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Toggle() => {
                self.data.completed = !self.data.completed;
                true // Indicate that the view should be re-rendered
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <li>
                <label id={self.data.id.clone()} class={if self.data.completed { "checked-item" } else { "" }}>
                    <input
                        type="checkbox"
                        checked={self.data.completed}
                        onclick={ctx.link().callback(|_| Msg::Toggle())}
                    />
                    { &self.data.title }
                </label>
            </li>
        }
    }
}
