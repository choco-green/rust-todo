#[derive(Clone, Debug)]
pub struct TodoItem {
    id: usize,
    text: String,
    completed: bool,
}

impl TodoItem {
    pub fn new(text: &str, id: usize) -> Self {
        Self {
            id,
            text: text.to_string(),
            completed: false,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_text(&self) -> &str {
        self.text.as_str()
    }

    pub fn get_completed(&self) -> bool {
        self.completed
    }

    pub fn toggle_completed(&mut self) {
        self.completed = !self.completed;
    }
}
