use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents the state machine of a Task (Pod).
///
/// The lifecycle flows as follows:
/// 1. `Pending`: The user submitted the task, but no Node has picked it up.
/// 2. `Scheduled`: The Scheduler found a Node, but the Worker hasn't started it yet.
/// 3. `Running`: The Docker container is successfully active on the Worker.
/// 4. `Completed`: The process exited with code 0.
/// 5. `Failed`: The process crashed or the image failed to pull.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum TaskStatus {
    Pending, // created but not scheduled
    Scheduled,
    Running,
    Complete,
    Failed,
}

/// A unit of work to be executed on the cluster.
///
/// This struct roughly corresponds to a Kubernetes "Pod" or a single Docker container definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique internal identifier
    pub id: Uuid,

    /// Human-readable name (e.g., "nginx-prod")
    pub name: String,

    /// The Docker image to run (e.g., "postgres:13")
    pub image: String,

    /// Memory requirement in MB
    pub memory: i32,

    /// CPU requirement in cores (e.g., 0.5 for half a core)
    pub cpu: f32,

    /// Environment variables to inject into the container.
    pub env: HashMap<String, String>,

    /// Status
    pub status: TaskStatus,

    /// Created time of the task
    pub created_at: DateTime<Utc>,

    /// Started time of the task
    pub started_at: Option<DateTime<Utc>>,

    /// The ID of the Worker Node where this task is assigned
    /// This is `None` when the task is in `Pending` state
    pub node_id: Option<String>, // the node where this task is running

    /// The actual Docker Container ID returned by the Docker Deamon.
    pub container_id: Option<String>,
}

impl Task {
    /// Creates a new Task with default resource limits.
    pub fn new(name: String, image: String) -> Self {
        Task {
            id: Uuid::new_v4(),
            name,
            image,
            memory: 256, // default to low memory footprint
            cpu: 0.5,    // default to half a core
            env: HashMap::new(),
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            started_at: None,
            node_id: None,
            container_id: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_task_defaults() {
        let task = Task::new("test-task".to_string(), "alpine:latest".to_string());
        assert_eq!(task.name, "test-task");
        assert_eq!(task.image, "alpine:latest");
        assert_eq!(task.status, TaskStatus::Pending);
        assert!(task.node_id.is_none());
        assert!(task.container_id.is_none());
    }

    #[test]
    fn test_unique_ids() {
        let task1 = Task::new("t1".to_string(), "img".to_string());
        let task2 = Task::new("t2".to_string(), "img".to_string());
        assert_ne!(task1.id, task2.id);
    }
}