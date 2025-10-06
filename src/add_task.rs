use std::time::SystemTime;

use crate::task::{Status, Task, Tasks};

pub(crate) fn add_task(tasks: &mut Tasks, description: String) {
    let now: SystemTime = SystemTime::now();
    let task: Task = Task {
        id: tasks.next_id,
        description: description,
        status: Status::Todo,
        created_at: now,
        updated_at: now,
    };

    println!(
        "Task successfully added: ID: {}, Description: {}",
        tasks.next_id, task.description
    );
    tasks.tasks.insert(tasks.next_id, task);
    tasks.next_id += 1;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_create_task() {
        let mut tasks: Tasks = Tasks {
            next_id: 1,
            tasks: HashMap::new(),
        };

        add_task(&mut tasks, String::from("this is a test"));
        assert_eq!(2, tasks.next_id);
        assert_eq!(1, tasks.tasks.len());
        add_task(&mut tasks, String::from("test2"));
        assert_eq!(3, tasks.next_id);
        assert_eq!(2, tasks.tasks.len());

        let now: SystemTime = SystemTime::now();

        let t1: &Task = tasks.tasks.get(&1).unwrap();
        let t2: &Task = tasks.tasks.get(&2).unwrap();

        assert_eq!(1, t1.id);
        assert_eq!(String::from("this is a test"), t1.description);
        assert_eq!(Status::Todo, t1.status);
        assert!(t1.created_at <= now && t1.updated_at <= now);
        assert!(t1.created_at == t1.updated_at);

        assert_eq!(2, t2.id);
        assert_eq!(String::from("test2"), t2.description);
        assert_eq!(Status::Todo, t2.status);
        assert!(t1.created_at <= t2.created_at && t1.updated_at <= t2.updated_at);
        assert!(t2.created_at == t2.updated_at);
    }
}
