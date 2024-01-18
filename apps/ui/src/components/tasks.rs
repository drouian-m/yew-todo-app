use yew::prelude::*;

use crate::store::Store;

pub struct TasksComponent {
    store: Store,
}

pub enum Msg {
    Toggle(String),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub store: Store,
}

impl Component for TasksComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            store: ctx.props().store.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Toggle(id) => {
                self.store.toggle_task(id);
            }
        }
        true // Indicate that the view should be re-rendered
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let tasks = self.store.get_tasks();
        html! {
          <ul id="task-list" class="todo-list">
            {
                tasks.iter().map(|item| html! {
                    <li key={format!("task-{}", item.id)}>
                        <label id={item.id.clone()} class={if item.completed { "checked-item" } else { "" }}>
                            <input
                                type="checkbox"
                                checked={item.completed}
                                onclick={ctx.link().callback(|_| Msg::Toggle(item.id.clone()))}
                            />
                            { &item.title }
                        </label>
                    </li>
                }).collect::<Html>()
            }
          </ul>
        }
    }
}
