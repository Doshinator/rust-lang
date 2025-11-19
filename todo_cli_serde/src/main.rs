use std::{env, fs::{self, read_to_string}};

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub complete: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToDoList {
    pub tasks: Vec<Task>,
}


impl ToDoList {
    pub fn load(filename: &str) -> Self {
        let data = read_to_string(filename)
            .unwrap_or_else(|_| {
                "[]".to_string()
            });

        serde_json::from_str(&data)
            .unwrap_or(ToDoList {
                tasks: vec![]
            })
    }

    pub fn save(&self, filename: &str) {
        let data = serde_json::to_string_pretty(&self)
            .unwrap();

        fs::write(filename, data)
            .expect("Unable to write file");
    }

    pub fn add_task(&mut self, title: String) {
        let task = Task {
            id: self.tasks.last().map_or(1, |n| n.id + 1),
            title,
            description: format!("tip toe, by the window: {}", self.tasks.last().map_or(1, |n| n.id + 1)),
            complete: false,
        };
        self.tasks.push(task);
    }

    pub fn mark_done(&mut self, id: u32) -> bool {
        if let Some(task) = self.tasks
            .iter_mut()
            .find(|t| t.id == id) {
                task.complete = true;
                true
            }
            else {
                false
            }
    }

    pub fn delete_task(&mut self, id: u32) -> bool {
        let len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        len != self.tasks.len()
    }

    pub fn list_tasks(&self) {
        for task in &self.tasks {
            println!("[{}] {} - {}", 
            if task.complete { 
                "x" 
            } else { 
                " " 
            }, 
            task.id,
            task.title);
        }
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut todo = ToDoList::load("tasks.json");

    match args.get(1).map(|s| s.as_str()) {
        Some("add") => {
            if let Some(title) = args.get(2) {
                todo.add_task(title.to_string());
                todo.save("tasks.json");
                println!("Task added: {}", title);
            }
        },
        Some("list") => todo.list_tasks(),
        Some("done") => {
            if let Some(id_str) = args.get(2) {
                if let Ok(id) = id_str.parse::<u32>() {
                    if todo.mark_done(id) {
                        todo.save("tasks.json");
                        println!("Task {} marked done", id);
                    } else {
                        println!("Task {} not found", id);
                    }
                }
            }
        },
        Some("delete") => {
            if let Some(id_str) = args.get(2) {
                if let Ok(id) = id_str.parse::<u32>() {
                    if todo.delete_task(id) {
                        todo.save("tasks.json");
                        println!("Task {} deleted", id);
                    }
                    else {
                        println!("Task {} not found", id);
                    }
                }
            }
        },
        _ => println!("Usage: add <task> | list | done <id> | delete <id>"),
    }

}
