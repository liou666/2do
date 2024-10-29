use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Seek, SeekFrom, Write},
    path::Path,
};

use crate::utils::{check_db_file, get_db_path};

#[derive(Debug)]
pub struct Todo {
    pub id: u64,
    pub content: String,
}

pub struct Database {
    pub file: File,
}

impl Database {
    pub fn open() -> Database {
        let _ = check_db_file();
        let path = get_db_path();

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
        Database { file }
    }
    pub fn add(&mut self, todo: Todo) -> Result<(), io::Error> {
        let content = format!("{} {}", todo.id, todo.content);
        writeln!(self.file, "{}", content)
    }

    pub fn list(&self) -> Vec<Todo> {
        BufReader::new(&self.file)
            .lines()
            .filter(|line| line.is_ok())
            .map(|line| {
                let line = line.unwrap();
                let mut parts = line.split_whitespace();
                let id = parts.next().unwrap().parse().unwrap();
                let content = parts.next().unwrap().to_string();
                Todo { id, content }
            })
            .collect()
    }

    pub fn remove(&mut self, id: u64) -> Result<(), io::Error> {
        let todos = self.list();
        let target = todos.iter().find(|todo| todo.id == id);

        match target {
            None => Err(io::Error::new(io::ErrorKind::NotFound, "Todo not found")),
            Some(_) => {
                self.file.seek(SeekFrom::Start(0)).expect("");
                self.file.set_len(0).expect("");
                for todo in todos {
                    if todo.id != id {
                        let _ = self.add(todo);
                    }
                }
                Ok(())
            }
        }
    }
}
