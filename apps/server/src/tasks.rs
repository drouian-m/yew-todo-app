use types::Task;

#[derive(Clone)]
pub struct Tasks {
    pub tasks: Vec<Task>,
}

impl Tasks {
    pub fn new() -> Self {
        Tasks { tasks: Vec::new() }
    }

    pub fn list(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn create(&mut self, title: &str) -> Task {
        let created_task = Task::new(title.to_string());
        self.tasks.push(created_task.clone());
        created_task
    }
}
