// src/config.rs
#[derive(Debug, Clone)]
pub struct AgentConfig {
    pub message_limit: usize,
    pub learning_capacity: usize,
}

impl Default for AgentConfig {
    fn default() -> Self {
        AgentConfig {
            message_limit: 100,
            learning_capacity: 1000,
        }
    }
}
