use std::sync::{Arc, Mutex};

use actix_cors::Cors;
use actix_web::{
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};

use serde::Deserialize;
use server::{tasks::Tasks, utils::load_demo};

struct State {
    pub tasks: Arc<Mutex<Tasks>>,
}

#[derive(Deserialize)]
struct CreateTaskRequest {
    title: String,
}

#[get("/tasks")]
async fn list_tasks(data: web::Data<State>) -> impl Responder {
    let tasks = data.tasks.lock().unwrap();
    let items: &Vec<server::model::task::Task> = tasks.list();
    web::Json(items.clone())
}

#[post("/tasks")]
async fn create_task(data: web::Data<State>, req: web::Json<CreateTaskRequest>) -> impl Responder {
    let mut tasks = match data.tasks.lock() {
        Ok(tasks) => tasks,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let created_task = tasks.create(&req.title);

    HttpResponse::Created().json(created_task)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        let cors = Cors::default();
        let mut tasks = Tasks::new();
        load_demo::load_demo(&mut tasks);
        let state = Data::new(State {
            tasks: Arc::new(Mutex::new(tasks)),
        });
        App::new()
            .app_data(state.clone())
            .wrap(cors)
            .service(list_tasks)
            .service(create_task)
    })
    .bind(("127.0.0.1", 8081))?
    .run();
    println!("Application running on http://localhost:{}", 8081);
    server.await
}