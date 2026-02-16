mod tasks;
use tasks::{Priority, create_task, mark_completed};

fn main() {
    let mut todo_list: Vec<tasks::models::Task> = Vec::new();
    // High priority tasks
    if let Ok(t) = create_task(1, "Install Rust", Priority::Hight, Some("Use rustup")) {
        todo_list.push(t);
    }
    if let Ok(t) = create_task(2, "Learn Modules", Priority::Medium, None) {
        todo_list.push(t);
    }
}
