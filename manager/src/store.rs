use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use common::{OrchError, Task, TaskStatus};

pub type SharedState = Arc<TaskStore>;

pub struct TaskStore {
    pub tasks: RwLock<HashMap<String, Task>>,
}

impl TaskStore {
    pub fn new() -> Self {
       Self {
           tasks: RwLock::new(HashMap::new()),
       }
    }

    pub fn add_task(&self, task: Task) -> Result<(), OrchError> {
        let mut task_write = self.tasks.write()
            .map_err(|e| OrchError::TaskStoreError(format!("Failed to unlock the task: {}", e)))?;

        task_write.insert(task.name.clone(), task);

        Ok(())
    }

    pub fn list_tasks(&self) -> Result<Vec<Task>, OrchError> {
        let mut task_read = self.tasks.read()
            .map_err(|e| OrchError::TaskStoreError(format!("Failed to unlock the task: {}", e)))?;
        let list = task_read.values().cloned().collect();

        Ok(list)
    }

    pub fn get_task(&self, id: Uuid) -> Result<Option<Task>, OrchError> {
        let task_read = self.tasks.read()
            .map_err(|e| OrchError::TaskStoreError(format!("Failed to unlock the task: {}", e)))?;

        let task = task_read.get(&id.to_string()).cloned();

        Ok(task)
    }

    pub fn update_status(&self, id: Uuid, status: TaskStatus, container_id: Option<String>) -> Result<bool, OrchError> {
        let mut task_write = self.tasks.write()
        .map_err(|e| OrchError::TaskStoreError(format!("Failed to unlock the task: {}", e)))?;

        if let Some(task) = task_write.get_mut(&id.to_string()) {
            task.status = status;
            if container_id.is_some() {
                task.container_id = container_id;
            }

            return Ok(true);
        }

        Ok(false)
    }
}