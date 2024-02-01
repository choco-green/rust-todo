use std::io;

use crossterm::{
    cursor, queue,
    style::{self, Print},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

use crate::{todo_list::TodoList, view::View};

pub fn update_screen<W: io::Write>(w: &mut W, todos: &TodoList, view: &View) -> io::Result<()> {
    queue!(
        w,
        style::ResetColor,
        Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0),
    )?;
    w.flush()?;

    match view {
        View::Active => {
            queue!(w, Print("Active Todos:\n\r"))?;
            todos.get_todos(&View::Active).iter().for_each(|todo| {
                if let Some(selected) = &todos.selected_todo_item {
                    if selected.get_id() == todo.get_id() {
                        queue!(w, Print("> ")).unwrap();
                    }
                }
                queue!(
                    w,
                    Print(format!("({}){}{}", todo.get_id(), " [ ] ", todo.get_text())),
                    Print("\n\r")
                )
                .unwrap();
            });
        }
        View::Completed => {
            queue!(w, Print("Completed Todos:\n\r"))?;
            todos.get_todos(&View::Completed).iter().for_each(|todo| {
                if let Some(selected) = &todos.selected_todo_item {
                    if selected.get_id() == todo.get_id() {
                        queue!(w, Print("> ")).unwrap();
                    }
                }
                queue!(
                    w,
                    Print(format!("({}){}{}", todo.get_id(), " [x] ", todo.get_text())),
                    Print("\n\r")
                )
                .unwrap();
            });
        }
    }

    debug(todos)?;
    w.flush()?;
    Ok(())
}

fn debug(todos: &TodoList) -> io::Result<()> {
    disable_raw_mode()?;
    dbg!(todos);
    enable_raw_mode()?;
    Ok(())
}
