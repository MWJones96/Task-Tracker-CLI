use clap::Parser;
use serde::Deserialize;
use std::{
    fs::{File, read_to_string},
    io::BufWriter,
    path::Path,
    time::SystemTime,
};

use serde_json::json;

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
struct Data {
    task_count: u32,
    tasks: Vec<Task>,
}

fn main() {
    let args = CliArgs::parse();

    let path = Path::new("tasks.json");
    if !path.exists() {
        let data = json!({ "task_count": 0, "tasks": [] });
        let file = File::create(path).unwrap();
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &data).unwrap();
    }

    let contents = read_to_string(path).unwrap();
    let data: Data = serde_json::from_str(&contents).unwrap();
}
