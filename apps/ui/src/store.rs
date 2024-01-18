use std::collections::HashSet;

use types::Task;

#[derive(PartialEq, Clone)]
pub struct Store {
    tasks: Vec<Task>,
}

impl Store {
    pub fn new(tasks: Option<Vec<Task>>) -> Self {
        Self {
            tasks: match tasks {
                Some(tasks) => tasks,
                None => vec![],
            },
        }
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn set_tasks(&mut self, tasks: Vec<Task>) {
        // Create a set of ids from the new tasks for faster lookup
        let task_ids: HashSet<_> = tasks.iter().map(|task| task.id.clone()).collect();

        for task in &mut self.tasks {
            if let Some(found) = tasks.iter().find(|t| t.id == task.id) {
                task.completed = found.completed;
            }
        }
        // Collect tasks that are not in self.tasks and append them
        let new_tasks: Vec<Task> = tasks
            .into_iter()
            .filter(|t| !task_ids.contains(&t.id))
            .collect();
        self.tasks.extend(new_tasks);
    }

    pub fn toggle_task(&mut self, task_id: String) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.completed = !task.completed;
        }
    }
}
