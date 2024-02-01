pub mod cli;
pub mod todo_item;
pub mod todo_list;
pub mod view;

use crate::cli::update_screen;
use crate::todo_list::TodoList;
use crate::view::View;

use crossterm::event::{self, Event, KeyCode};
use std::io::{self, stdout};

fn main() -> io::Result<()> {
    let mut stdout = stdout();
    let mut todo_list = TodoList::new(Some(vec![
        "Learn Rust",
        "Make a game",
        "Make a todo app",
        "Go to sleep",
    ]));

    let mut view = View::Active;

    loop {
        update_screen(&mut stdout, &todo_list, &view)?;
        if let Event::Key(event) = event::read()? {
            match event.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('e') => view.toggle(),
                KeyCode::Char('s') => match todo_list.selected_todo_item {
                    Some(_) => todo_list.deselect_todo_item(),
                    None => todo_list.select_todo_item(&mut stdout),
                },
                KeyCode::Char('d') => todo_list.delete_todo_item(),
                KeyCode::Char('a') => todo_list.add_todo_item(&mut stdout),
                KeyCode::Char(' ') => todo_list.toggle_completed(),
                _ => {}
            }
        }
    }
    Ok(())
}
