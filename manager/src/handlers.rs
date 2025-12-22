use serde::Deserialize;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use uuid::Uuid;
use common::{Task, TaskStatus};
use crate::store::SharedState;

#[derive(Deserialize)]
struct UpdateStatusRequest {
    status: TaskStatus,
    container_id: Option<String>,
}

pub async fn handle_connection(mut stream: TcpStream, store: SharedState) -> anyhow::Result<()> {
    let mut buffer = [0; 1024];

    let n = stream.read(&mut buffer).await?;
    if n == 0 {
        return Ok(());
    }

    let request = String::from_utf8_lossy(&buffer[..n]);

    if request.starts_with("GET /tasks") {
        handle_get_tasks(stream, store).await?;
    } else if request.starts_with("POST /tasks") {
        handle_post_task(stream, request.to_string(), store).await?;
    } else if request.starts_with("PUT /tasks") {
        handle_update_status(stream, request.to_string(), store).await?; 
    } else {
        let response = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n";
        stream.write_all(response.as_bytes()).await?;
    }

    Ok(())
}

async fn handle_get_tasks(mut stream: TcpStream, store: SharedState) -> anyhow::Result<()> {
    let mut tasks = store.list_tasks()?;
    tasks = tasks
        .into_iter()
        .filter(|t| t.status == TaskStatus::Scheduled)
        .collect::<Vec<Task>>();
    let body = serde_json::to_string(&tasks)?;

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    );

    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

async fn handle_post_task(mut stream: TcpStream, request: String, store: SharedState) -> anyhow::Result<()> {
    // check body (usually after new double line)
    if let Some(body) = request.split("\r\n\r\n").last() {
       // clean up any trailing null bytes from the TCP buffer
        let body = body.trim_matches(char::from(0));

        if let Ok(task_req) = serde_json::from_str::<serde_json::Value>(body) {
            let name = task_req["name"].as_str().unwrap_or("unnamed");
            let image = task_req["image"].as_str().unwrap_or("unnamed");

            let new_task = Task::new(name.to_string(), image.to_string());

            // acquire a write lock and save the task
            store.add_task(new_task.clone())?;

            let body = serde_json::to_string(&new_task)?;
            let response = format!(
                "HTTP/1.1 201 CREATED\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(response.as_bytes()).await?;
            return Ok(());
        }
    }

    let err_resp = "HTTP/1.1 400 BAD REQUEST\r\nContent-Length: 0\r\n\r\n";
    stream.write_all(err_resp.as_bytes()).await?;
    Ok(())
}

async fn handle_update_status(mut stream: TcpStream, request: String, store: SharedState) -> anyhow::Result<()> {
    // Basic extraction of ID from PUT /tasks/<id>/status
    let first_line = request.lines().next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    if parts.len() < 2 { return Ok(()); }

    let path = parts[1];
    let id = path.split('/').nth(2).unwrap_or("");
    let id_uuid = Uuid::parse_str(id)?;

    if let Some(body) = request.split("\r\n\r\n").last() {
        let body = body.trim_matches(char::from(0)).trim();
        if let Ok(update_req) = serde_json::from_str::<UpdateStatusRequest>(body) {
             store.update_status(id_uuid, update_req.status, update_req.container_id)?;
            let response = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(response.as_bytes()).await?;
            return Ok(());
        }
    }
    stream.write_all("HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes()).await?;
    Ok(())
}