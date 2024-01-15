use super::schema::tasks;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
}
