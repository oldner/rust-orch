/// The operational status of a worker node
pub enum NodeStatus {
    /// The node is online and sending heartbeats.
    Ready,
    /// The node has stopped heartbeats or is explicitly disabled
    NotReady,
}

pub struct Node {
    /// Unique identifier for the machine (e.g., "worker-01")
    pub id: String,
    pub name: String,
    pub ip_address: String,
    pub status: NodeStatus,
    pub total_memory: i32,
    pub total_cpu: f32,
    pub available_memory: i32,
    pub available_cpu: f32,
}

impl Node {
    pub fn new(name: String, total_memory: i32, total_cpu: f32) -> Self {
        Node {
            id: name.clone(),
            name,
            ip_address: "127.0.0.1".to_string(),
            status: NodeStatus::NotReady,
            total_memory,
            total_cpu,
            available_memory: total_memory,
            available_cpu: total_cpu,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_node() {
        let node = Node::new("worker-1".to_string(), 4096, 4.0);
        assert_eq!(node.name, "worker-1");
        assert_eq!(node.total_memory, 4096);
        assert_eq!(node.available_memory, 4096);
        assert!(matches!(node.status, NodeStatus::NotReady));
    }
}