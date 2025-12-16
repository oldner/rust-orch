use std::collections::HashMap;
use uuid::Uuid;
use worker::DockerClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting Worker Node..");

    // initialize docker client
    let docker = DockerClient::new().await?;
    println!("Connected to Docker Daemon");

    // simulate a task
    let task_id = Uuid::new_v4();
    let image = "nginx:latest";
    let mut env = HashMap::new();
    env.insert("IsRustOrch".to_string(), "true".to_string() );

    // start the container
    println!("Starting the container (Image: {})..", image);
    let container_id = docker.start_container(&task_id.to_string(), image, env).await?;
    println!("Started container (Image: {}).", image);

    println!("sleeping for 10 seconds...");
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    println!("stopping container...");
    docker.stop_container(&container_id).await?;
    println!("Container {} stopped and removed successfully.", container_id);

    Ok(())
}