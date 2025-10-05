use std::time::SystemTime;

enum Status {
    Todo,
    InProgress,
    Done,
}

struct Task {
    id: u32,
    description: String,
    status: Status,
    created_at: SystemTime,
    updated_at: SystemTime,
}

fn main() {
    println!("Hello, world!");
}
