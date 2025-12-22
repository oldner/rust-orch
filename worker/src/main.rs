use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;
use common::{Task, TaskStatus};
use worker::DockerClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let node_id = "worker-1";
    let manager_url = "http://localhost:3000";
    println!("Starting Worker {node_id}");

    // initialize docker client
    let docker = DockerClient::new().await?;
    println!("Worker: Connected to Docker Daemon");
    let http_client = reqwest::Client::new();

    loop {
        let resp = http_client.get(format!("{}/tasks", &manager_url)).send().await?;
        let tasks = resp.json::<Vec<Task>>().await?;
        let worker_tasks = tasks
          .into_iter()
          .filter(|t| t.node_id.as_deref() == Some(node_id))
          .collect::<Vec<Task>>();
        
        println!("Worker: tasks len: {}", worker_tasks.len());
       
        for task in worker_tasks {
           match docker.start_container(&task.id.to_string(), &task.image, HashMap::new()).await {
               Ok(container_id) => {
                   let update_payload = serde_json::json!({
                       "status": TaskStatus::Running,
                       "container_id": Some(container_id.to_string()),
                   });

                   let _ = http_client.put(format!("{}/tasks/{}/status", manager_url, task.id.to_string()))
                       .json(&update_payload)
                       .send()
                       .await;
                   
                   println!("Worker: Successfully started container {}", container_id);
               },
               Err(e) => {
                    eprintln!("Worker: Error starting container: {}", e);
                    let _ = http_client.put(format!("{}/tasks/{}/status", manager_url, task.id.to_string()))
                        .json(&serde_json::json!({"status": TaskStatus::Failed, "container_id": None::<String>}))
                        .send()
                        .await;
               }
           };
       }
        sleep(Duration::from_secs(5)).await;
    };
}