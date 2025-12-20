use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum OrchError {
    DockerError(String),
    TaskNotFound(String),
    NodeNotFound(String),
    SchedulerError(String),
    NetworkError(String),
    TaskStoreError(String),
}

impl std::fmt::Display for OrchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrchError::DockerError(msg) => write!(f, "Docker operation failed: {}", msg),
            OrchError::TaskNotFound(id) => write!(f, "Task not found: {}", id),
            OrchError::NodeNotFound(id) => write!(f, "Node not found: {}", id),
            OrchError::SchedulerError(msg) => write!(f, "Scheduler error: {}", msg),
            OrchError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            OrchError::TaskStoreError(msg) => write!(f, "Task store error: {}", msg),
        }
    }
}

impl std::error::Error for OrchError {}
