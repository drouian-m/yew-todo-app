use diesel::{associations::HasTable, prelude::*};
use types::Task as TaskType;

use crate::{
    models::*,
    schema::{self, tasks},
};

pub struct Tasks<'a> {
    pub db_conn: &'a mut SqliteConnection,
}

impl<'a> Tasks<'a> {
    pub fn new(db_conn: &'a mut SqliteConnection) -> Self {
        Tasks { db_conn }
    }

    pub fn list(&mut self) -> Vec<Task> {
        use schema::tasks::dsl::*;
        let items: Vec<Task> = tasks
            .select(Task::as_select())
            .load(self.db_conn)
            .expect("Error loading posts");
        items
    }

    pub fn create(&mut self, task_title: &str) -> Task {
        let to_create = TaskType::new(task_title.to_owned());
        let created_task = diesel::insert_into(tasks::table)
            .values(&NewTask {
                id: to_create.id,
                title: to_create.title,
                completed: to_create.completed,
                created_at: to_create.created_at,
            })
            .returning(Task::as_returning())
            .get_result(self.db_conn)
            .expect("Error saving new post");
        created_task
    }
}
