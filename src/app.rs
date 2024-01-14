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
        Task {
            title: "Buy groceries".to_owned(),
        },
        Task {
            title: "Finish work presentation".to_owned(),
        },
        Task {
            title: "Read a book".to_owned(),
        },
        Task {
            title: "Clean the house".to_owned(),
        },
        Task {
            title: "Plan a vacation".to_owned(),
        },
        Task {
            title: "Do some Rust stuffs".to_owned(),
        },
    ];
    html! {
        <div class="container">
            <h1>{ "Todo App" }</h1>
            <ul id="task-list" class="todo-list">
                {for tasks.iter().map(|item| html! { <TaskComponent title={item.title.clone()} />})}
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
