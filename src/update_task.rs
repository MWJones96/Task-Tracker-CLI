use std::time::SystemTime;

use crate::Status;
use crate::Task;

fn update_task(id: u32, new_description: &str) -> Task {
    Task {
        id: u32::MAX,
        description: String::from("test"),
        status: Status::Todo,
        created_at: SystemTime::now(),
        updated_at: SystemTime::now(),
        deleted: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(1 + 2, 3);
    }
}
