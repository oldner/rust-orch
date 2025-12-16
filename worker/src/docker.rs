use std::collections::HashMap;

use bollard::Docker;
use bollard::models::{Config, ContainerConfig, ContainerCreateBody};
use bollard::query_parameters::{CreateContainerOptions, CreateImageOptions, RemoveContainerOptions, StartContainerOptions, StopContainerOptions};
use futures_util::StreamExt;
use common::OrchError;

pub struct DockerClient {
    inner: Docker,
}

impl DockerClient {
    pub async fn new() -> Result<Self, OrchError> {
        let docker = Docker::connect_with_socket_defaults()
            .map_err(|e| OrchError::DockerError(format!("Failed to connect to Docker: {}", e)))?;
        Ok(Self { inner: docker })
    }

    pub async fn start_container(
        &self,
        task_id: &str,
        image: &str,
        env: HashMap<String, String>,
    ) -> Result<String, OrchError> {
        let container_name = format!("rust-orch-{}", task_id);

        // check if image exists locally, if not pull it.

        println!("Checking for image: {}", image);
        if self.inner.inspect_image(image).await.is_err() {
            println!("Image not found locally, pulling {}", image);
            let mut pull_stream = self.inner.create_image(
                    Some(CreateImageOptions {
                        from_image: Some(image.to_string()),
                        ..Default::default()
                    }),
                    None,
                    None,
                );

            // We use a while loop to consume the stream.
            // This avoids buffering all logs into memory
            while let Some(output) = pull_stream.next().await {
                match output {
                    Ok(info) => {
                        // TODO: add progress bar..
                        if let Some(status) = info.status {
                            println!("Docker: {}", status)
                        }
                    }
                    Err(e) => return Err(OrchError::DockerError(format!("Failed to pull image: {}", e))),
                }
            }
        }

        // Configure the container
        // We map the Task ID to the Container Name for easy lookup later.
        let config = ContainerCreateBody {
            image: Some(image.to_string()),
            env: Some(env.iter().map(|(k, v)| format!("{}={}", k, v)).collect()),
            ..Default::default()
        };

        // Create container
        let create_options = CreateContainerOptions {
            name: Some(container_name),
            ..Default::default()
        };

        let res = self.inner
            .create_container(Some(create_options), config)
            .await
            .map_err(|e| OrchError::DockerError(format!("Failed to start container: {}", e)))?;

        let container_id = res.id;

        self.inner
            .start_container(&container_id, None::<StartContainerOptions>)
            .await
            .map_err(|e| OrchError::DockerError(format!("Failed to start container: {}", e)))?;

        Ok(container_id)
    }

    pub async fn stop_container(&self, container_id: &str) -> Result<(), OrchError> {
        // Stop
        self.inner
            .stop_container(container_id, None::<StopContainerOptions>)
            .await
            .map_err(|e| OrchError::DockerError(format!("Failed to stop container: {}", e)))?;

        // Remove
        self.inner
            .remove_container(
                container_id,
                Some(RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                })
            )
            .await
            .map_err(|e| OrchError::DockerError(format!("Failed to remove container: {}", e)))?;
        
        Ok(())
    }
}
