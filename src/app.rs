use todo_app::components::task::TaskComponent;
use yew::prelude::*;
struct Task {
    // id: i32,
    title: String,
    // ended: bool,
}

#[function_component(App)]
pub fn app() -> Html {
    let tasks = vec![
        "Buy groceries".to_owned(),
        "Finish work presentation".to_owned(),
        "Read a book".to_owned(),
        "Clean the house".to_owned(),
        "Plan a vacation".to_owned(),
        "Do some Rust stuffs".to_owned(),
    ];
    html! {
        <div class="container">
            <h1>{ "Todo App" }</h1>
            <ul id="task-list" class="todo-list">
                {for tasks.iter().map(|item| html! { <TaskComponent title={item.clone()} />})}
            </ul>
        </div>
    }
}

// #[derive(Properties, PartialEq)]
// pub struct TaskProps {
//     pub title: String,
// }

// #[function_component]
// fn TaskComponent(props: &TaskProps) -> Html {
//     html! {
//         <li><span>{&props.title}</span></li>
//     }
// }
