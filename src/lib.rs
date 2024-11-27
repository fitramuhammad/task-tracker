use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Config {
    query: String,
    args: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get query string"),
        };

        let todo = match args.next() {
            Some(arg) => arg,
            None if query == "list" => "".to_string(),
            None => return Err("Didn't get todo"),
        };

        Ok(Config { query, args: todo })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Status {
    Done,
    Todo,
    InProgres,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Status::Done => write!(f, "done"),
            Status::InProgres => write!(f, "in-progress"),
            Status::Todo => write!(f, "todo"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Task {
    id: u8,
    task: String,
    status: Status,
}

impl Task {
    pub fn new(id: u8, task: String) -> Self {
        Task {
            id,
            task,
            status: Status::Todo,
        }
    }

    pub fn change_status(&mut self, status: &'static str) {
        match status {
            "done" => self.status = Status::Done,
            "in-progress" => self.status = Status::InProgres,
            "todo" => self.status = Status::Todo,
            _ => (),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    todos: Vec<Task>,
}

impl Todo {
    pub fn new() -> Self {
        Todo { todos: Vec::new() }
    }

    pub fn add(&mut self, todo: String) {
        let id = self.todos.len() as u8 + 1;
        let task = Task::new(id, todo);

        self.todos.push(task);
    }

    pub fn delete(&mut self, id: u8) {
        self.todos.retain(|x| x.id != id);
    }

    pub fn mark_in_progress(&mut self, id: u8) {
        let task = self.todos.iter_mut().find(|x| x.id == id).unwrap();
        task.change_status("in-progress");
    }

    pub fn mark_done(&mut self, id: u8) {
        let task = self.todos.iter_mut().find(|x| x.id == id).unwrap();
        task.change_status("done");
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = std::fs::read_to_string("todo.json").unwrap();

    let mut deserialize: Todo = serde_json::from_str(&content).unwrap();

    match config.query.as_str() {
        "add" => {
            deserialize.add(config.args);
            let _ = std::fs::write(
                "todo.json",
                serde_json::to_string(&deserialize).unwrap().as_bytes(),
            );
        }
        "update" => todo!(),
        "delete" => {
            deserialize.delete(config.args.parse().unwrap());
            let _ = std::fs::write(
                "todo.json",
                serde_json::to_string(&deserialize).unwrap().as_bytes(),
            );
        }
        "mark-in-progress" => {
            deserialize.mark_in_progress(config.args.parse().unwrap());
            let _ = std::fs::write(
                "todo.json",
                serde_json::to_string(&deserialize).unwrap().as_bytes(),
            );
        }
        "mark-done" => {
            deserialize.mark_done(config.args.parse().unwrap());
            let _ = std::fs::write(
                "todo.json",
                serde_json::to_string(&deserialize).unwrap().as_bytes(),
            );
        }
        "list" => match config.args.as_str() {
            "done" => {
                println!("Tasks: ");
                for i in deserialize
                    .todos
                    .iter()
                    .filter(|x| x.status == Status::Done)
                    .collect::<Vec<&Task>>()
                {
                    println!("{}. {} {}", i.id, i.task, i.status.to_string());
                }
            }
            "in-progress" => {
                println!("Tasks: ");
                for i in deserialize
                    .todos
                    .iter()
                    .filter(|x| x.status == Status::InProgres)
                    .collect::<Vec<&Task>>()
                {
                    println!("{}. {} {}", i.id, i.task, i.status.to_string());
                }
            }
            "todo" => {
                println!("Tasks: ");
                for i in deserialize
                    .todos
                    .iter()
                    .filter(|x| x.status == Status::Todo)
                    .collect::<Vec<&Task>>()
                {
                    println!("{}. {} {}", i.id, i.task, i.status.to_string());
                }
            }
            _ => {
                println!("Tasks: ");
                for i in deserialize.todos.iter() {
                    println!("{}. {} {}", i.id, i.task, i.status.to_string());
                }
            }
        },
        _ => {
            eprintln!("invalid");
            std::process::exit(1);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_todo() {
        let todo = Task::new(1, "makan".to_string());

        assert_eq!(
            todo,
            Task {
                id: 1,
                task: "makan".to_string(),
                status: Status::InProgres
            }
        );
    }
}
