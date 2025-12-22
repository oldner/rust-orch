use std::time::Duration;
use tokio::time::sleep;
use common::{OrchError, TaskStatus};
use crate::store::SharedState;

pub async fn run_scheduler_task(store: SharedState) -> Result<(), OrchError> {
    println!("Starting scheduler...");
    loop {
        // find all tasks currently `pending`
        let tasks = store.list_tasks()?;
        let pending_tasks: Vec<_> = tasks
            .into_iter()
            .filter(|t| t.status == TaskStatus::Pending)
            .collect();

        // run a loop check each task and assign to a node
        for task in pending_tasks {
            let target_node = "worker-1".to_owned();

            store.assign_node(task.id, target_node.clone())?;
            println!("task {} is assigned to {}", task.id, target_node);
        }

        sleep(Duration::from_secs(5)).await;
    }
}