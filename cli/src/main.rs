use std::fs::read;
use clap::{Parser, Subcommand};
use prettytable::{format, row, Table};
use serde_json::json;
use common::Task;

#[derive(Parser)]
#[command(name = "orch")]
#[command(about = "Rust-Orch: a simple container orchestrator CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// start a new task in the cluster
    Run {
        /// name of the task
        name: String,
        /// docker image to use
        image: String,
    },
    List,
}

const MANAGER_URL: &str = "http://127.0.0.1:3000";

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let client = reqwest::blocking::Client::new();

    match &cli.command {
        Commands::Run { name, image } => {
            println!("Submitting task '{}' with image '{}'...", name, image);

            let payload = json!({
                "name": name,
                "image": image,
            });

            let response = client.post(format!("{}/tasks", MANAGER_URL)).json(&payload).send()?;

            if response.status().is_success() {
                let task: Task = response.json()?;
                println!("Task '{}' successfully submitted.", task.name);
            } else {
                eprintln!("Error submitting task: {}", response.status());
            }
        }
        Commands::List => {
            let response = client.get(format!("{}/tasks", MANAGER_URL)).send()?;

            if response.status().is_success() {
                let tasks: Vec<Task> = response.json()?;

                if tasks.is_empty() {
                    println!("No tasks found in the cluster.");
                    return Ok(());
                }

                let mut table = Table::new();
                table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
                table.set_titles(row!["NAME", "ID", "IMAGE", "STATUS", "NODE", "CONTAINER"]);

                for t in tasks {
                    let node = t.node_id.unwrap_or_else(|| "-".to_string());
                    let container = t.container_id.map(|id| id[..8].to_string()).unwrap_or_else(|| "-".to_string());

                    table.add_row(row![
                        t.name,
                        t.id.to_string()[..8],
                        t.image,
                        format!("{:?}", t.status),
                        node,
                        container,
                    ]);
                }

                table.printstd();
            } else {
                eprintln!("Error listing tasks: {}", response.status());
            }
        }
    }

    Ok(())
}
