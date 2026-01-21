use chrono::Local;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    description: String,
    status: String,
    created_at: String,
    updated_at: String,
}

struct Tasks {
    tasks: Vec<Task>,
    path: String,
}

impl Tasks {
    fn load(path: String) -> Tasks {
        if !std::path::Path::new(&path).exists() {
            fs::write(&path, "[]").expect(&format!("Could not create {}", path));
        }

        let contents: Vec<Task> = serde_json::from_str(
            fs::read_to_string("tasks.json")
                .expect("cant read file")
                .as_str(),
        )
        .expect("could not parse JSON");

        return Tasks {
            tasks: contents,
            path,
        };
    }

    fn create(&mut self, args: Vec<String>) {
        let new_desc = args[2..].join(" ");
        let task = Task {
            id: create_id(&self.tasks),
            description: new_desc,
            status: "todo".to_string(),
            created_at: now_datetime(),
            updated_at: now_datetime(),
        };
        self.tasks.push(task);
        self.save();
    }

    fn update(&mut self, args: Vec<String>) {
        let update_desc: String = args[3..].join(" ");
        let update_id: u32 = args[2].parse().expect("Task ID needs to be a number");
        let index = self.tasks.iter_mut().position(|task| task.id == update_id);
        match index {
            Some(i) => {
                self.tasks[i].description = update_desc;
                self.tasks[i].updated_at = now_datetime();
                self.save();
                println!("Task updated");
            }
            None => {
                println!("Task not found");
            }
        }
    }

    fn save(&self) {
        let json =
            serde_json::to_string_pretty(&self.tasks).expect("failed to convert list to json");
        fs::write(&self.path, json).expect("failed to write to file");
    }

    fn delete(&mut self, args: Vec<String>) {
        let delete_id: u32 = args[2].parse().expect("Task ID needs to be a number");
        self.tasks.retain(|task| task.id != delete_id);
        self.save();
    }

    fn list(&self, args: Vec<String>) {
        for task in &self.tasks {
            /*check if there are more than one
            argument before trying to access
            index 2 */
            if args.len() <= 2 || args[2] == task.status {
                println!("{} | {} | {}", task.id, task.description, task.status);
            }
        }
    }

    fn mark_in_progress(&mut self, args: Vec<String>) {
        let update_id: u32 = args[2].parse().expect("Task ID needs to be a number");
        for task in &mut self.tasks {
            if task.id == update_id {
                task.status = "in-progress".to_string();
                task.updated_at = now_datetime();
                break;
            }
        }
        self.save();
    }

    fn mark_done(&mut self, args: Vec<String>) {
        let update_id: u32 = args[2].parse().expect("Task ID needs to be a number");
        for task in &mut self.tasks {
            if task.id == update_id {
                task.status = "done".to_string();
                task.updated_at = now_datetime();
                break;
            }
        }
        self.save();
    }
}

fn create_id(tasks: &Vec<Task>) -> u32 {
    tasks.iter().map(|t| t.id + 1).max().unwrap_or(0)
}

fn now_datetime() -> String {
    Local::now().format("%Y-%m-%dT%H:%M:%S").to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut tasks = Tasks::load("tasks.json".to_string());

    // match for different options. list, add, etc.
    match args[1].as_str() {
        "list" => {
            tasks.list(args);
        }
        "add" => {
            tasks.create(args);
        }
        "update" => {
            tasks.update(args);
        }
        "remove" => {
            tasks.delete(args);
        }
        "mark-in-progress" => {
            tasks.mark_in_progress(args);
        }
        "mark-done" => {
            tasks.mark_done(args);
        }

        _ => {
            println!("unknown command. try again");
        }
    }
}
