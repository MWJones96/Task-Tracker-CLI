use std::time::SystemTime;

mod add_task;
mod delete_task;
mod update_task;

#[derive(Debug, PartialEq, Eq)]
enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug)]
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

fn main() {
    println!("Hello, world!");
}
