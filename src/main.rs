use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{File, read_to_string},
    io::{BufReader, BufWriter},
    path::Path,
    time::SystemTime,
};

use serde_json::json;

mod add_task;
mod delete_task;
mod update_task;

#[derive(Debug, PartialEq, Eq, Deserialize)]
enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Deserialize)]
struct Task {
    id: u32,
    description: String,
    status: Status,
    created_at: SystemTime,
    updated_at: SystemTime,
    deleted: bool,
}

fn mark_in_progress(id: u32) {}
fn mark_done(id: u32) {}
fn list() {}
fn list_done() {}
fn list_todo() {}
fn list_in_progress() {}

#[derive(Debug, Deserialize)]
struct Data {
    task_count: u32,
    tasks: Vec<Task>,
}

fn main() {
    let path = Path::new("tasks.json");
    if !path.exists() {
        let data = json!({ "task_count": 0, "tasks": [] });
        let file = File::create(path).unwrap();
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &data).unwrap();
    }

    let contents = read_to_string(path).unwrap();
    let data: Data = serde_json::from_str(&contents).unwrap();

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() < 2 {
        println!("Valid commands: add, update, delete, mark-in-progress, mark-done, list");
        return;
    }
    match &args[1][..] {
        "add" => {
            println!("add");
        }
        "update" => {
            println!("update");
        }
        "delete" => {
            println!("delete");
        }
        "mark-in-progress" => {
            println!("mark-in-progress");
        }
        "mark-done" => {
            println!("mark-done");
        }
        "list" => {
            println!("list");
        }
        _ => {
            println!("not valid")
        }
    }
}
