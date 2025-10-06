use clap::Parser;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::{File, read_to_string},
    io::BufWriter,
    path::Path,
    time::SystemTime,
};

use serde_json::{Map, json};

use crate::args::CliArgs;
mod add_task;
mod args;
mod delete_task;
mod list_task;
mod mark_task;
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

#[derive(Debug, Deserialize)]
struct Tasks {
    next_id: u32,
    tasks: HashMap<u32, Task>,
}

fn get_json_data() -> Tasks {
    let path = Path::new("tasks.json");
    if !path.exists() {
        let data = json!({ "next_id": 1, "tasks": {} });
        let file = File::create(path).unwrap();
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &data).unwrap();
    }

    let contents = read_to_string(path).unwrap();
    serde_json::from_str(&contents).unwrap()
}

fn main() {
    let args = CliArgs::parse();
    let data: Tasks = get_json_data();
    println!("{:?}", data)
}
