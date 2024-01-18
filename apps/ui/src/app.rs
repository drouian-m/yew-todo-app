use types::Task;
use ui::{
    components::{create_task::CreateTaskComponent, task::TaskComponent},
    services::tasks_service::list_tasks,
    store::Store,
};
use yew::prelude::*;

// pub struct App {
//     tasks: Vec<Task>,
// }

// pub enum Msg {
//     AddTask,
//     ReloadTasks(Vec<Task>),
// }

// impl Component for App {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_: &Context<Self>) -> Self {
//         Self { tasks: vec![] }
//     }

//     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::AddTask => {
//                 // Clone link to be used inside the async block
//                 let link = ctx.link().clone();
//                 wasm_bindgen_futures::spawn_local(async move {
//                     let result = list_tasks().await.unwrap();
//                     link.send_message(Msg::ReloadTasks(result));
//                 });
//             }
//             Msg::ReloadTasks(tasks) => {
//                 self.tasks = tasks;
//                 ctx.link()
//                     .call_component_method(self, "update", &vec![&self.tasks]);
//             }
//         }
//         true
//     }

//     fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
//         if first_render {
//             let link = ctx.link().clone();
//             wasm_bindgen_futures::spawn_local(async move {
//                 let result = list_tasks().await.unwrap();
//                 link.send_message(Msg::ReloadTasks(result));
//             });
//         }
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         html! {
//             <div class="container">
//                 <h1>{ "Todo App" }</h1>
//                 <CreateTaskComponent task_created={ctx.link().callback(|_: String| Msg::AddTask)}/>
//                 <ul id="task-list" class="todo-list">
//                     {for self.tasks.iter().map(|item| html! { <TaskComponent task={item.clone()} />})}
//                 </ul>
//             </div>
//         }
//     }
// }

#[function_component(App)]
pub fn app() -> Html {
    let new_tasks = use_state(|| vec![]);
    let store = Store::new(None);
    {
        let new_tasks = new_tasks.clone();
        use_effect_with((), move |_| {
            let new_tasks = new_tasks.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_tasks: Vec<Task> = list_tasks().await.unwrap();
                store.set_tasks(fetched_tasks);
                // store.new_tasks.set(fetched_tasks);
            });
            || ()
        });
    }

    let on_task_created: Callback<String> = {
        let new_tasks = new_tasks.clone();
        Callback::from(move |title: String| {
            // let new_tasks = new_tasks.clone();
            let message = format!("task {} created !", title);
            web_sys::console::log_1(&message.into()); // if uncommented will print
                                                      // new_tasks.set(vec![]);
            wasm_bindgen_futures::spawn_local(async move {
                let result = list_tasks().await.unwrap();
                store.set_tasks(result);
            });
        })
    };

    html! {
        <div class="container">
            <h1>{ "Todo App" }</h1>
            <CreateTaskComponent task_created={on_task_created}/>
            <TasksComponent store={store}/>
        </div>
    }
}
