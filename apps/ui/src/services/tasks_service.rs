use anyhow::Error;
use gloo_net::http::Request;
use serde::Serialize;
use serde_json::to_string;
use std::result::Result;
use types::Task;

#[derive(Serialize)]
struct CreateTaskRequest {
    title: String,
}

pub async fn list_tasks() -> Result<Vec<Task>, Error> {
    let tasks: Vec<Task> = Request::get("/tasks").send().await?.json().await?;
    Ok(tasks)
}

pub async fn create_task(title: String) -> Result<(), Error> {
    let json_body = to_string(&CreateTaskRequest { title })?;
    let response = Request::post("/tasks")
        .header("Content-Type", "application/json")
        .body(json_body)?
        .send()
        .await?;

    if response.ok() {
        Ok(())
    } else {
        Err(anyhow::Error::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to send request",
        )))
    }
}
