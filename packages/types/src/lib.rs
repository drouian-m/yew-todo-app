use chrono::{NaiveDateTime, Utc};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
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

#[cfg(test)]
mod tests {
    use crate::Task;

    #[test]
    fn create_task() {
        let created = Task::new("demo".to_owned());
        assert_eq!(created.title, "demo".to_owned());
        assert_eq!(created.completed, false);
    }
}
