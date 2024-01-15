use gloo_net::http::Request;
use types::Task;
use ui::{components::task::TaskComponent, services::tasks_service::list_tasks};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let new_tasks = use_state(|| vec![]);
    {
        let new_tasks = new_tasks.clone();
        use_effect_with((), move |_| {
            let new_tasks = new_tasks.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_tasks: Vec<Task> = list_tasks().await.unwrap();
                new_tasks.set(fetched_tasks);
            });
            || ()
        });
    }

    html! {
        <div class="container">
            <h1>{ "Todo App" }</h1>
            <ul id="task-list" class="todo-list">
                {for new_tasks.iter().map(|item| html! { <TaskComponent task={item.clone()} />})}
            </ul>
        </div>
    }
}
