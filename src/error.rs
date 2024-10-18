use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub struct TaskerRunError {}

impl Display for TaskerRunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Error while running the Tasker - Unknown command provided!!")
    }
}

impl Error for TaskerRunError {}
