use std::time::SystemTime;

use crate::task::{Task, Tasks};

pub(crate) fn update_task(tasks: &mut Tasks, id: u32, new_description: String) -> bool {
    if tasks.tasks.contains_key(&id) {
        let mut task: &mut Task = tasks.tasks.get_mut(&id).unwrap();
        task.description = new_description.to_owned();
        println!("Task with ID {id} updated successfully to \"{new_description}\"");
        return true;
    }

    println!("Task with ID {id} does not exist");
    false
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::task::Task;

    use super::*;

    #[test]
    fn test_update_task() {
        let now: SystemTime = SystemTime::now();
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

        assert_eq!(false, update_task(&mut tasks, 2, "test2".to_string()));

        assert_eq!(2, tasks.next_id);
        assert_eq!("test", tasks.tasks.get(&1).unwrap().description);

        assert_eq!(
            true,
            update_task(&mut tasks, 1, "changed description".to_string())
        );

        assert_eq!(2, tasks.next_id);
        assert_eq!(
            "changed description",
            tasks.tasks.get(&1).unwrap().description
        );
    }
}
