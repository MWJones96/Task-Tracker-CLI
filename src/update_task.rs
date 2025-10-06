use std::time::SystemTime;

fn update_task(id: u32, new_description: &str) -> crate::task::Task {
    crate::task::Task {
        id: u32::MAX,
        description: String::from("test"),
        status: crate::task::Status::Todo,
        created_at: SystemTime::now(),
        updated_at: SystemTime::now(),
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
