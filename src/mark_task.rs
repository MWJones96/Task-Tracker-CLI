use crate::task::Tasks;

pub(crate) fn mark_in_progress(tasks: &mut Tasks, id: u32) -> bool {
    if tasks.tasks.contains_key(&id) {
        tasks.tasks.get_mut(&id).unwrap().status = crate::task::Status::InProgress;
        println!("Task {id} marked in progress");
        return true;
    }

    println!("Task {id} does not exist");
    false
}

pub(crate) fn mark_done(tasks: &mut Tasks, id: u32) -> bool {
    if tasks.tasks.contains_key(&id) {
        tasks.tasks.get_mut(&id).unwrap().status = crate::task::Status::Done;
        println!("Task {id} marked done");
        return true;
    }

    println!("Task {id} does not exist");
    false
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, time::SystemTime};

    use crate::task::{Status, Task};

    use super::*;

    #[test]
    fn test_mark_in_progress() {
        let now: SystemTime = SystemTime::now();

        let mut tasks: Tasks = Tasks {
            next_id: 2,
            tasks: HashMap::from([(
                1,
                Task {
                    id: 1,
                    description: "test".to_owned(),
                    status: crate::task::Status::Todo,
                    created_at: now,
                    updated_at: now,
                },
            )]),
        };

        assert_eq!(false, mark_in_progress(&mut tasks, 2));

        assert_eq!(Status::Todo, tasks.tasks.get(&1).unwrap().status);
        assert_eq!(true, mark_in_progress(&mut tasks, 1));
        assert_eq!(Status::InProgress, tasks.tasks.get(&1).unwrap().status);
    }

    #[test]
    fn test_mark_done() {
        let now: SystemTime = SystemTime::now();

        let mut tasks: Tasks = Tasks {
            next_id: 2,
            tasks: HashMap::from([(
                1,
                Task {
                    id: 1,
                    description: "test".to_owned(),
                    status: crate::task::Status::Todo,
                    created_at: now,
                    updated_at: now,
                },
            )]),
        };

        assert_eq!(false, mark_done(&mut tasks, 2));

        assert_eq!(Status::Todo, tasks.tasks.get(&1).unwrap().status);
        assert_eq!(true, mark_done(&mut tasks, 1));
        assert_eq!(Status::Done, tasks.tasks.get(&1).unwrap().status);
    }
}
