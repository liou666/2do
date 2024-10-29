use std::io::Error;

use crate::database::{Database, Todo};

pub fn add(db: &mut Database, content: Option<String>) -> Result<(), Error> {
    match content {
        None => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Content is required",
            ));
        }
        Some(content) => {
            let id = db.list().last().map_or(1, |todo| todo.id + 1);
            let _ = db.add(Todo {
                id,
                content: content.clone(),
            });
            println!("[DONE] Added todo: {}", content);
            Ok(())
        }
    }
}

pub fn remove(db: &mut Database, id: Option<u64>) -> Result<(), Error> {
    if id.is_none() {
        println!("You need to specify the id of the todo item.");
        std::process::exit(1);
    }
    let _ = db.remove(id.unwrap())?;
    println!("[DON] Removed todo with id: {}", id.unwrap());
    Ok(())
}

pub fn list(db: &Database) -> Result<(), Error> {
    let todos = db.list();
    if todos.is_empty() {
        eprintln!("No records. You can add one with `todo add [content]`");
        std::process::exit(1);
    }
    for todo in todos {
        println!("{} {}", todo.id, todo.content);
    }
    Ok(())
}
