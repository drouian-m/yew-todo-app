use yew::prelude::*;

pub struct TaskComponent {
    id: String,
    title: String,
    ended: bool,
}

pub enum Msg {
    Toggle(),
}

#[derive(Properties, PartialEq)]
pub struct TaskProps {
    pub title: String,
}

impl Component for TaskComponent {
    type Message = Msg;
    type Properties = TaskProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            id: chrono::offset::Local::now().to_string(),
            title: ctx.props().title.clone(),
            ended: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Toggle() => {
                self.ended = !self.ended;
                true // Indicate that the view should be re-rendered
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <li>
                <label class={if self.ended { "checked-item" } else { "" }}>
                    <input
                        type="checkbox"
                        checked={self.ended}
                        onclick={ctx.link().callback(|_| Msg::Toggle())}
                    />
                    { &self.title }
                </label>
            </li>
        }
    }
}
