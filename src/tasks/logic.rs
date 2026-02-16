use crate::tasks::models::{Priority, Task};

pub fn create_task(id: u32, name: &str, p: Priority, desc: Option<&str>) -> Result<Task, String> {
    if name.is_empty() {
        return Err(String::from("Name cannot be Empty"));
    }
    Ok(Task {
        id,
        name: name.to_string(),
        priority: p,
        description: desc.map(|s| s.to_string()),
        is_done: false,
    })
}

pub fn mark_completed(task: &mut Task) {
    task.is_done = true;
}
