use std::env;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_json;

fn main() {
    let args: Vec<String> = env::args().collect();

    let task_desc = &args[2];

    struct Task {
        id: i32,
        description: String,
        status: String,
        created_at: String,
        updated_at: String,
    }

    fn create_id {

    }

      match args[1].as_str() {
        "list" => {
            let contents =
                fs::read_to_string("tasks.json").expect("Should have been able to read the file");
            println!("{contents}")
        }
        "add" => {
            fs::write("tasks.json", task).expect("Unable to write file");
        }
        _ => {
            println!("unknown command. try again")
        }
    }
}
