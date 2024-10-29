use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Seek, SeekFrom, Write},
    path::Path,
};

#[derive(Debug)]
pub struct Todo {
    pub id: u64,
    pub content: String,
}

pub struct Command {
    pub file: File,
}

impl Command {
    pub fn open(path: &str) -> Command {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(Path::new(path))
            .unwrap();
        Command { file }
    }
    pub fn add(&mut self, todo: Todo) {
        let content = format!("{} {}", todo.id, todo.content);
        writeln!(self.file, "{}", content).unwrap();
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

    pub fn remove(&mut self, id: u64) -> Option<()> {
        let todos = self.list();
        let target = todos.iter().find(|todo| todo.id == id);

        println!("{:?}", target);

        if target.is_none() {
            return None;
        }
        self.file.seek(SeekFrom::Start(0)).expect("dd");
        self.file.set_len(0).expect("sssddddd");
        for todo in todos {
            if todo.id != id {
                self.add(todo);
            }
        }
        Some(())
    }
}
