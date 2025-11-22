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

fn create_task(tasks: &Vec<Task>, new_desc: String) -> Task {
    Task {
        id: create_id(tasks),
        description: new_desc,
        status: "todo".to_string(),
        created_at: task_create_datetime(),
        updated_at: task_create_datetime(),
    }
}

fn create_id(tasks: &Vec<Task>) -> u32 {
    let mut new_id: u32 = 0;
    for task in tasks {
        if task.id > new_id {
            new_id = task.id;
        }
    }
    new_id + 1
}

fn task_create_datetime() -> String {
    Local::now().format("%Y-%m-%dT%H:%M:%S").to_string()
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(&tasks).expect("failed to add task to list");
    fs::write("tasks.json", json).expect("failed to write to file");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // read file and then parse to json
    let contents = fs::read_to_string("tasks.json").expect("cant read file");

    let mut tasks: Vec<Task> = serde_json::from_str(&contents).expect("could not parse JSON");

    // match for different options. list, add, etc.
    match args[1].as_str() {
        "list" => {
            for task in &tasks {
                /*check if there are more than one
                argument before trying to access
                index 2 */
                if args.len() <= 2 || args[2] == task.status {
                    println!("{} | {} | {}", task.id, task.description, task.status);
                }
            }
        }
        "add" => {
            let new_desc = &args[2..].join(" ");
            let new_task = create_task(&tasks, new_desc.clone());
            tasks.push(new_task);
            save_tasks(&tasks);
        }
        "update" => {
            let task_update_desc: &String = &args[3..].join(" ");
            let update_id: u32 = args[2].parse().expect("Task ID needs to be a number");
            for task in &mut tasks {
                if task.id == update_id {
                    task.description = task_update_desc.clone();
                    task.updated_at = task_create_datetime();
                    break;
                }
            }
            save_tasks(&tasks);
        }
        "remove" => {
            let delete_id: u32 = args[2].parse().expect("Task ID needs to be a number");
            tasks.retain(|task| task.id != delete_id);
            save_tasks(&tasks);
        }
        "mark-in-progress" => {
            let update_id: u32 = args[2].parse().expect("Task ID needs to be a number");
            for task in &mut tasks {
                if task.id == update_id {
                    task.status = "in-progress".to_string();
                    task.updated_at = task_create_datetime();
                    break;
                }
            }
            save_tasks(&tasks);
        }
        "mark-done" => {
            let update_id: u32 = args[2].parse().expect("Task ID needs to be a number");
            for task in &mut tasks {
                if task.id == update_id {
                    task.status = "done".to_string();
                    task.updated_at = task_create_datetime();
                    break;
                }
            }
            save_tasks(&tasks);
        }

        _ => {
            println!("unknown command. try again");
        }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_task() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
