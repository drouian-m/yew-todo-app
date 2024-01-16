use types::Task;
use ui::{
    components::{create_task::CreateTaskComponent, task::TaskComponent},
    services::tasks_service::list_tasks,
};
use yew::prelude::*;

pub struct App {
    tasks: Vec<Task>,
    loading_stage: ComponentLoadingStage,
}

pub enum ComponentLoadingStage {
    Loading,
    Success,
    Error,
}
pub enum Msg {
    ReloadTasks(Vec<Task>),
}

impl App {
    async fn refresh_tasks(&mut self) {
        self.tasks = list_tasks().await.unwrap();
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            tasks: vec![],
            loading_stage: ComponentLoadingStage::Loading,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ReloadTasks(tasks) => {
                self.tasks = tasks;
            }
        }
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let link = ctx.link().clone();
            wasm_bindgen_futures::spawn_local(async move {
                let result = list_tasks().await.unwrap();
                link.send_message(Msg::ReloadTasks(result));
            });
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        web_sys::console::log_1(&"Reload component...".into());

        // let on_task_created = ctx.link().callback(|_| {
        //     let link = ctx.link().clone();
        //     wasm_bindgen_futures::spawn_local(async move {
        //         let result = list_tasks().await.unwrap();
        //         link.send_message(Msg::ReloadTasks(result));
        //     });
        // });

        // let on_task_created: Callback<String> = Callback::from(move |_: String| {
        //     let link = ctx.link().clone();
        //     wasm_bindgen_futures::spawn_local(async move {
        //         let result = list_tasks().await.unwrap();
        //         link.send_message(Msg::ReloadTasks(result));
        //     });
        //     // web_sys::console::log_1(&greeting.into()); // if uncommented will print
        // });

        html! {
            <div class="container">
                <h1>{ "Todo App" }</h1>
                <CreateTaskComponent task_created={on_task_created}/>
                <ul id="task-list" class="todo-list">
                    {for self.tasks.iter().map(|item| html! { <TaskComponent task={item.clone()} />})}
                </ul>
            </div>
        }
    }
}

// #[function_component(App)]
// pub fn app() -> Html {
//     let new_tasks = use_state(|| vec![]);
//     {
//         let new_tasks = new_tasks.clone();
//         use_effect_with((), move |_| {
//             let new_tasks = new_tasks.clone();
//             wasm_bindgen_futures::spawn_local(async move {
//                 let fetched_tasks: Vec<Task> = list_tasks().await.unwrap();
//                 new_tasks.set(fetched_tasks);
//             });
//             || ()
//         });
//     }

//     let on_task_created: Callback<String> = Callback::from(move |title: String| {
//         let message = format!("task {} created !", title);
//         web_sys::console::log_1(&message.into()); // if uncommented will print
//     });

//     html! {
//         <div class="container">
//             <h1>{ "Todo App" }</h1>
//             <CreateTaskComponent task_created={on_task_created}/>
//             <ul id="task-list" class="todo-list">
//                 {for new_tasks.iter().map(|item| html! { <TaskComponent task={item.clone()} />})}
//             </ul>
//         </div>
//     }
// }
