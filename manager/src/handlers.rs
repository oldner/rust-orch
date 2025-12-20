use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use common::Task;
use crate::store::SharedState;

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
    } else {
        let response = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n";
        stream.write_all(response.as_bytes()).await?;
    }

    Ok(())
}

async fn handle_get_tasks(mut stream: TcpStream, store: SharedState) -> anyhow::Result<()> {
    let tasks = store.list_tasks();
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