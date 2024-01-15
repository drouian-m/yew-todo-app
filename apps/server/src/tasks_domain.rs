use anyhow::Error;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use types::Task as TaskType;

use crate::{
    models::*,
    schema::{self, tasks},
};

#[derive(Clone)]
pub struct Tasks {
    pub db: Pool<ConnectionManager<SqliteConnection>>,
}

impl Tasks {
    pub fn new(db: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        Tasks { db }
    }

    pub fn list(&mut self) -> Result<Vec<Task>, Error> {
        use schema::tasks::dsl::*;
        let mut connection = self.db.get()?;
        let items: Vec<Task> = tasks
            .select(Task::as_select())
            .load(&mut connection)
            .expect("Error loading posts");
        Ok(items)
    }

    pub fn create(&mut self, task_title: &str) -> Result<Task, Error> {
        let mut connection = self.db.get()?;

        let to_create = TaskType::new(task_title.to_owned());
        let created_task = diesel::insert_into(tasks::table)
            .values(&NewTask {
                id: to_create.id,
                title: to_create.title,
                completed: to_create.completed,
                created_at: to_create.created_at,
            })
            .returning(Task::as_returning())
            .get_result(&mut connection)
            .expect("Error saving new post");
        Ok(created_task)
    }
}
