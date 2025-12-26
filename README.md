
# Rust-Orch 🦀

A lightweight, distributed container orchestrator built from scratch in Rust to understand the "under the hood" mechanics of Kubernetes.

This project implements a decentralized architecture consisting of a **Control Plane (Manager)**, a **Data Plane (Worker)**, and a **CLI Interface**, communicating over a custom-built asynchronous HTTP layer.

## 🏗️ Architecture

The system is divided into four main components:

-   **`manager`**: The "Brain" of the cluster. It manages the global state of tasks using thread-safe primitives (`Arc`, `RwLock`), hosts a manual HTTP server, and runs a background **Scheduler** loop that reconciles desired state with actual state.

-   **`worker`**: The "Muscle" of the cluster. A lightweight agent that polls the Manager, interacts with the local Linux Docker socket via the `bollard` crate, and manages container lifecycles.

-   **`cli`**: A user-friendly command-line interface built with `clap` to interact with the cluster API.

-   **`common`**: A shared library containing the domain models (`Task`, `Node`), shared error types, and serialization logic.


## 🚀 Key Features & Concepts

-   **Zero-Framework HTTP**: The Manager's API server is built using raw `tokio::net::TcpListener` and manual byte-parsing to demonstrate deep understanding of the HTTP protocol.

-   **Async Concurrency**: Extensive use of the `tokio` runtime for non-blocking I/O and background task spawning.

-   **Thread Safety**: Implementation of `Arc` (Atomic Reference Counting) and `RwLock` (Read-Write Locks) to manage shared state across concurrent API requests and the scheduler loop.

-   **Docker Integration**: Direct communication with the Docker Engine API to pull images and manage container execution.

-   **State Machine Logic**: Tasks transition through a lifecycle (`Pending` -> `Scheduled` -> `Running` -> `Completed`/`Failed`) managed by a reconciliation loop.


## 🛠️ Prerequisites

-   **Rust**: Latest stable version.

-   **Docker**: Must be running on the host machine where the `worker` is deployed.

-   **Cargo Workspace**: The project is structured as a workspace for optimized compilation.


## 🚦 Getting Started

### 1. Start the Manager (Control Plane)

```
cargo run -p manager

```

The manager will start listening on `127.0.0.1:3000` and initiate the background scheduler.

### 2. Start the Worker (Data Plane)

```
cargo run -p worker

```

The worker will connect to the local Docker daemon and begin polling the manager for assigned work.

### 3. Use the CLI

Open a new terminal to submit and monitor tasks.

**Run a new container:**

```
cargo run -p cli -- run my-web-server nginx:latest

```

**List cluster status:**

```
cargo run -p cli -- list

```

## 📂 Project Structure

```
.
├── common/        # Shared Domain Models & Logic
├── manager/       # API Server, In-memory Store, & Scheduler
│   ├── src/store.rs     # Thread-safe State Management
│   ├── src/handlers.rs  # Raw HTTP Request Handling
│   └── src/scheduler.rs # Background Reconciliation Loop
├── worker/        # Docker Agent & Polling Logic
│   └── src/docker.rs    # Bollard / Docker API Wrapper
└── cli/           # Clap-based Terminal Interface

```

## 🗺️ Roadmap

-   [ ] **Persistence**: Move from In-memory `HashMap` to a persistent store (AOF or SQLite).

-   [ ] **Advanced Scheduling**: Implement resource-aware scheduling (CPU/RAM thresholds).

-   [ ] **Health Checks**: Implement active "liveness" probes from the worker back to the manager.

-   [ ] **Multi-node Networking**: Support for workers running on different physical/virtual machines.


## 📝 License

This project is open-source and available under the MIT License.