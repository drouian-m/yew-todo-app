use anyhow::Error;
use gloo_net::http::Request;
use types::Task;

pub async fn list_tasks() -> Result<Vec<Task>, Error> {
    let tasks: Vec<Task> = Request::get("/tasks").send().await?.json().await?;
    Ok(tasks)
}
