use std::time::SystemTime;

use crate::Status;
use crate::Task;

fn add_task(id: u32, description: String) -> Task {
    let now: SystemTime = SystemTime::now();
    Task {
        id,
        description,
        status: Status::Todo,
        created_at: now,
        updated_at: now,
        deleted: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_task() {
        let t: Task = add_task(1, String::from("this is a test"));
        let t2: Task = add_task(2, String::from("test2"));

        let now: SystemTime = SystemTime::now();

        assert_eq!(1, t.id);
        assert_eq!(String::from("this is a test"), t.description);
        assert_eq!(Status::Todo, t.status);
        assert!(t.created_at <= now && t.updated_at <= now);
        assert!(t.created_at == t.updated_at);
        assert_eq!(false, t.deleted);

        assert_eq!(2, t2.id);
        assert_eq!(String::from("test2"), t2.description);
        assert_eq!(Status::Todo, t2.status);
        assert!(t.created_at <= t2.created_at && t.updated_at <= t2.updated_at);
        assert!(t2.created_at == t2.updated_at);
        assert_eq!(false, t2.deleted);
    }
}
