use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};

use crate::tasks_domain::Tasks;

pub fn load_demo(conn: Pool<ConnectionManager<SqliteConnection>>, tasks: &mut Tasks) {
    use crate::schema::tasks;
    use diesel::RunQueryDsl;
    let mut connection = conn.get().unwrap();
    diesel::delete(tasks::table).execute(&mut connection);

    tasks.create("Buy groceries");
    tasks.create("Plan a vacation");
}
