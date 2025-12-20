use std::sync::Arc;
use tokio::net::TcpListener;
use crate::store::TaskStore;

mod handlers;
mod store;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "127.0.0.1:3000";
    println!("Manager listening on {}", addr);

    // shared between threads so we use Arc
    let shared_store = Arc::new(TaskStore::new());

    let listener = TcpListener::bind(addr).await?;

    loop {
        let (socket, _) = listener.accept().await?;

        let store = Arc::clone(&shared_store);

        tokio::spawn(async move {
            if let Err(e) = handlers::handle_connection(socket, store).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}
