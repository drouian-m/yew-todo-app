use std::sync::{Arc, Mutex};

use actix_web::{
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};

use serde::Deserialize;
use server::{establish_connection, tasks_domain::Tasks, utils::load_demo};

struct State {
    pub tasks: Arc<Mutex<Tasks>>,
}

#[derive(Deserialize)]
struct CreateTaskRequest {
    title: String,
}

#[get("/tasks")]
async fn list_tasks(data: web::Data<State>) -> impl Responder {
    let mut tasks = match data.tasks.lock() {
        Ok(tasks) => tasks,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let items = tasks.list();
    HttpResponse::Ok().json(items.unwrap().clone())
}

#[post("/tasks")]
async fn create_task(data: web::Data<State>, req: web::Json<CreateTaskRequest>) -> impl Responder {
    let mut tasks = match data.tasks.lock() {
        Ok(tasks) => tasks,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let created_task = tasks.create(&req.title);

    HttpResponse::Created().json(created_task.unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = establish_connection();
    let mut tasks = Tasks::new(connection.clone());
    load_demo::load_demo(connection.clone(), &mut tasks);
    let server = HttpServer::new(move || {
        let state = Data::new(State {
            tasks: Arc::new(Mutex::new(tasks.clone())),
        });
        App::new()
            .app_data(state.clone())
            .service(list_tasks)
            .service(create_task)
    })
    .bind(("127.0.0.1", 8081))?
    .run();
    println!("Application running on http://localhost:{}", 8081);
    server.await
}
