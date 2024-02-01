pub enum View {
    Active,
    Completed,
}

impl View {
    pub fn toggle(&mut self) {
        match self {
            View::Active => *self = View::Completed,
            View::Completed => *self = View::Active,
        }
    }
}
