use crate::task::Tasks;

pub(crate) fn list(tasks: &Tasks) -> Vec<String> {
    let mut list: Vec<String> = vec![];
    for (id, task) in &tasks.tasks {
        let desc = &task.description;
        let status = match task.status {
            crate::task::Status::Todo => "To Do",
            crate::task::Status::InProgress => "In Progress",
            crate::task::Status::Done => "Done",
        };
        list.push(format!(
            "ID: {id} | Description: \"{desc}\" | Status: {status}"
        ));
    }
    return list;
}

pub(crate) fn list_done(tasks: &Tasks) -> Vec<String> {
    let mut list: Vec<String> = vec![];
    for (id, task) in &tasks.tasks {
        if task.status != crate::task::Status::Done {
            continue;
        }

        let desc = &task.description;
        list.push(format!("ID: {id} | Description: \"{desc}\" | Status: Done"));
    }
    return list;
}

pub(crate) fn list_todo(tasks: &Tasks) -> Vec<String> {
    let mut list: Vec<String> = vec![];
    for (id, task) in &tasks.tasks {
        if task.status != crate::task::Status::Todo {
            continue;
        }

        let desc = &task.description;
        list.push(format!(
            "ID: {id} | Description: \"{desc}\" | Status: To Do"
        ));
    }
    return list;
}

pub(crate) fn list_in_progress(tasks: &Tasks) -> Vec<String> {
    let mut list: Vec<String> = vec![];
    for (id, task) in &tasks.tasks {
        if task.status != crate::task::Status::InProgress {
            continue;
        }

        let desc = &task.description;
        list.push(format!(
            "ID: {id} | Description: \"{desc}\" | Status: In Progress"
        ));
    }
    return list;
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, time::SystemTime};

    use crate::task::Task;

    use super::*;

    #[test]
    fn test_list() {
        let now: SystemTime = SystemTime::now();
        let mut tasks: Tasks = Tasks {
            next_id: 4,
            tasks: HashMap::from([
                (
                    1,
                    Task {
                        id: 1,
                        description: "test".to_owned(),
                        status: crate::task::Status::Todo,
                        created_at: now,
                        updated_at: now,
                    },
                ),
                (
                    2,
                    Task {
                        id: 2,
                        description: "test".to_owned(),
                        status: crate::task::Status::InProgress,
                        created_at: now,
                        updated_at: now,
                    },
                ),
                (
                    3,
                    Task {
                        id: 3,
                        description: "test".to_owned(),
                        status: crate::task::Status::Done,
                        created_at: now,
                        updated_at: now,
                    },
                ),
            ]),
        };

        assert_eq!(3, list(&tasks).len());
        assert_eq!(1, list_todo(&tasks).len());
        assert_eq!(1, list_in_progress(&tasks).len());
        assert_eq!(1, list_done(&tasks).len());
    }
}
