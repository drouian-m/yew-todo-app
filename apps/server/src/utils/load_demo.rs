use crate::tasks::Tasks;

pub fn load_demo(tasks: &mut Tasks) {
    tasks.create("Buy groceries");
    tasks.create("Plan a vacation");
}
