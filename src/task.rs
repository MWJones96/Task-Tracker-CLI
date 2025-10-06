use std::{collections::HashMap, fmt::Display, time::SystemTime};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Task {
    pub(crate) id: u32,
    pub(crate) description: String,
    pub(crate) status: Status,
    pub(crate) created_at: SystemTime,
    pub(crate) updated_at: SystemTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Tasks {
    pub(crate) next_id: u32,
    pub(crate) tasks: HashMap<u32, Task>,
}
