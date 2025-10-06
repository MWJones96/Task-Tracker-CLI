use crate::task::Tasks;

pub(crate) fn delete_task(tasks: &mut Tasks, id: u32) -> bool {
    if tasks.tasks.contains_key(&id) {
        tasks.tasks.remove(&id);
        println!("Task with ID {id} has been deleted");
        return true;
    }

    println!("Task with ID {id} doesn't exist");
    false
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, time::SystemTime};

    use crate::task::Task;

    use super::*;

    #[test]
    fn test_delete_task() {
        let now = SystemTime::now();
        let mut tasks: Tasks = Tasks {
            next_id: 2,
            tasks: HashMap::from([(
                1,
                Task {
                    id: 1,
                    description: "test".to_owned(),
                    status: crate::task::Status::Done,
                    created_at: now,
                    updated_at: now,
                },
            )]),
        };

        assert_eq!(false, delete_task(&mut tasks, 2));
        assert_eq!(2, tasks.next_id);
        assert_eq!(1, tasks.tasks.len());

        assert_eq!(true, delete_task(&mut tasks, 1));
        assert_eq!(2, tasks.next_id);
        assert_eq!(0, tasks.tasks.len());
    }
}
