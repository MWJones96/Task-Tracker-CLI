use clap::Parser;
use std::{
    fs::{File, read_to_string},
    io::BufWriter,
    path::Path,
};

use serde_json::json;

use crate::{
    add_task::add_task,
    list_task::{list, list_done, list_in_progress, list_todo},
    mark_task::{mark_done, mark_in_progress},
    task::Tasks,
};
use crate::{delete_task::delete_task, update_task::update_task};

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
        let data = json!({ "nextId": 1, "tasks": {} });
        let file = File::create(path).unwrap();
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &data).unwrap();
    }

    let contents = read_to_string(path).unwrap();
    serde_json::from_str(&contents).unwrap()
}

fn write_json_data(tasks: &Tasks) {
    let path = Path::new("tasks.json");
    let file = File::create(path).unwrap();

    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, tasks).unwrap();
}

fn main() {
    let args = args::CliArgs::parse();
    let mut tasks: task::Tasks = get_json_data();

    match args.command {
        args::Command::Add { description } => {
            add_task(&mut tasks, description);
        }
        args::Command::Update { id, description } => {
            update_task(&mut tasks, id, description);
        }
        args::Command::Delete { id } => {
            delete_task(&mut tasks, id);
        }
        args::Command::MarkInProgress { id } => {
            mark_in_progress(&mut tasks, id);
        }
        args::Command::MarkDone { id } => {
            mark_done(&mut tasks, id);
        }
        args::Command::List { list_command } => match list_command {
            Some(args::ListCommand::Done) => {
                let joined = list_done(&tasks).join("\n");
                println!("{}", joined);
            }
            Some(args::ListCommand::Todo) => {
                let joined = list_todo(&tasks).join("\n");
                println!("{}", joined);
            }
            Some(args::ListCommand::InProgress) => {
                let joined = list_in_progress(&tasks).join("\n");
                println!("{}", joined);
            }
            None => {
                let joined = list(&tasks).join("\n");
                println!("{}", joined);
            }
        },
    }

    write_json_data(&tasks);
}
