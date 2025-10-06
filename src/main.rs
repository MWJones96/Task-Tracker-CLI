use clap::Parser;
use std::{
    fs::{File, read_to_string},
    io::BufWriter,
    path::Path,
};

use serde_json::json;

mod add_task;
mod args;
mod delete_task;
mod list_task;
mod mark_task;
mod task;
mod update_task;

fn get_json_data() -> task::Tasks {
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
    let args = args::CliArgs::parse();
    let data: task::Tasks = get_json_data();
    println!("{:?}", data);

    match args.command {
        args::Command::Add { description } => {}
        args::Command::Update { id, description } => {}
        args::Command::Delete { id } => {}
        args::Command::MarkInProgress { id } => {}
        args::Command::MarkDone { id } => {}
        args::Command::List { list_command } => {}
        _ => println!("Hi"),
    }
}
