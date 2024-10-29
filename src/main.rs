use lib::{Command, Todo};

mod lib;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut command = Command::open(".todo");
    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Please specify a todo");
                return;
            }
            let id = command.list().len() as u64 + 1;
            let content = &args[2];
            command.add(Todo {
                id,
                content: content.to_string(),
            });
            println!("[DONE] Added todo: {}", content);
        }
        "rm" => {
            if args.len() < 3 {
                println!("Please specify a todo id");
                return;
            }
            let id = args[2].parse().unwrap();
            let result = command.remove(id);
            match result {
                None => {
                    println!("Todo with id: {} not found", id);
                    return;
                }
                Some(_) => {
                    println!("[DON] Removed todo with id: {}", id);
                }
            }
        }
        "ls" => {
            let todos = command.list();
            for todo in todos {
                println!("{} {}", todo.id, todo.content);
            }
        }
        _ => {
            println!("Invalid command");
        }
    };
}
