use chrono::{NaiveDateTime, Utc};
use nanoid::nanoid;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct Task {
    id: String,
    title: String,
    completed: bool,
    created_at: NaiveDateTime,
}

impl Task {
    pub fn new(title: String) -> Self {
        Task {
            id: nanoid!(),
            title,
            completed: false,
            created_at: Utc::now().naive_utc(),
        }
    }
}
