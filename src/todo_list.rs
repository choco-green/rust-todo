use std::io::{self, stdin};

use crossterm::style::Print;
use crossterm::{queue, terminal};

use crate::todo_item::TodoItem;
use crate::view::View;

#[derive(Clone, Debug)]
pub struct TodoList {
    pub todos: Vec<TodoItem>,
    pub selected_todo_item: Option<TodoItem>,
}

impl TodoList {
    pub fn new(todo_items: Option<Vec<&str>>) -> Self {
        match todo_items {
            Some(todo_items) => Self {
                todos: todo_items
                    .iter()
                    .enumerate()
                    .map(|(id, text)| TodoItem::new(text, id))
                    .collect(),
                selected_todo_item: None,
            },
            None => Self {
                todos: vec![],
                selected_todo_item: None,
            },
        }
    }

    pub fn get_todos(&self, view: &View) -> Vec<&TodoItem> {
        match view {
            View::Active => self
                .todos
                .iter()
                .filter(|todo| !todo.get_completed())
                .collect(),
            View::Completed => self
                .todos
                .iter()
                .filter(|todo| todo.get_completed())
                .collect(),
        }
    }

    pub fn add_todo_item<W: io::Write>(&mut self, w: &mut W) {
        let input = prompt(w, "Enter the new todo item: ");

        if input.is_empty() {
            return;
        }

        let id = self.todos.len();
        self.todos.push(TodoItem::new(&input, id));
    }

    pub fn select_todo_item<W: io::Write>(&mut self, w: &mut W) {
        let input = match prompt(w, "Enter the ID: ").parse::<usize>() {
            Ok(valid_id) => valid_id,
            Err(_) => return,
        };

        if let Some(todo_item) = self.todos.iter().find(|todo| todo.get_id() == input) {
            self.selected_todo_item = Some(todo_item.clone());
        };
    }

    pub fn delete_todo_item(&mut self) {
        if let Some(todo_item) = &self.selected_todo_item {
            let index = self
                .todos
                .iter()
                .position(|todo| todo.get_id() == todo_item.get_id())
                .unwrap();
            self.todos.remove(index);
        }
        self.deselect_todo_item();
    }

    pub fn deselect_todo_item(&mut self) {
        self.selected_todo_item = None;
    }

    pub fn toggle_completed(&mut self) {
        if let Some(selected_todo) = &self.selected_todo_item {
            if let Some(todo) = self
                .todos
                .iter_mut()
                .find(|todo| todo.get_id() == selected_todo.get_id())
            {
                todo.toggle_completed();
            }
            self.deselect_todo_item();
        }
    }
}

fn prompt<W>(w: &mut W, prompt: &str) -> String
where
    W: io::Write,
{
    terminal::disable_raw_mode().unwrap();

    queue!(w, Print(prompt)).unwrap();
    w.flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    terminal::enable_raw_mode().unwrap();

    input.trim().to_string()
}
