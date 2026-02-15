#[derive(Debug)]
pub enum Priority {
    Hight,
    Medium,
    Low,
}

#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub priority: priority,
    pub description: Option<String>,
    pub is_done: bool,
}
