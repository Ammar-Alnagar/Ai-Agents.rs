// src/agents/registry.rs
use crate::agents::{Agent, AgentType};

pub struct AgentRegistry;

impl AgentRegistry {
    pub fn create_agent(agent_type: AgentType) -> Box<dyn Agent> {
        match agent_type {
            AgentType::Basic => Box::new(BasicAgent::default()),
            AgentType::Learning => Box::new(LearningAgent::default()),
        }
    }
}
