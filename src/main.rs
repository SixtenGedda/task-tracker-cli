use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let task_desc = &args[2];

    #[derive(Serialize, Deserialize)]
    struct Task {
        id: u32,
        description: String,
        status: String,
        created_at: String,
        updated_at: String,
    }

    // read file and then parse to json
    let contents = fs::read_to_string("tasks.json").expect("cant read file");

    let tasks: Vec<Task> = serde_json::from_str(&contents).expect("could not parse JSON");

    fn create_id(tasks: &Vec<Task>) -> u32 {
        let mut new_id: u32 = 0;
        for task in tasks {
            if task.id > new_id {
                new_id = task.id;
            }
        }
        new_id + 1
    }

    fn add_task() {}

    // switch case for different options. list, add, etc.
    match args[1].as_str() {
        "list" => {
            for task in &tasks {
                println!("{} | {} | {}", task.id, task.description, task.status);
            }
            println!("suggest new id: {}", create_id(&tasks))
        }
        //"add" => {
        //    fs::write("tasks.json", Task).expect("Unable to write file");
        //}
        _ => {
            println!("unknown command. try again")
        }
    }
}
