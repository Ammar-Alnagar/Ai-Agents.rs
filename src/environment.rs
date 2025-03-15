use std::collections::HashMap;
use anyhow::{Result, anyhow};
use crate::message::Message;

pub struct Environment {
    agents: HashMap<usize, Agent>,
    message_queue: Vec<Message>,
    current_tick: u32,
    next_agent_id: usize,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            agents: HashMap::new(),
            message_queue: Vec::new(),
            current_tick: 0,
            next_agent_id: 0,
        }
    }

    pub fn add_agent(&mut self, mut agent: Agent) {
        let agent_id = self.next_agent_id;
        agent.set_id(agent_id);
        self.agents.insert(agent_id, agent);
        self.next_agent_id += 1;
    }

    pub fn send_message(&mut self, message: Message) -> Result<()> {
        if !self.agents.contains_key(&message.receiver) {
            return Err(anyhow!("Invalid receiver ID: {}", message.receiver));
        }
        self.message_queue.push(message);
        Ok(())
    }

    pub fn run(&mut self, ticks: u32) -> Result<()> {
        for _ in 0..ticks {
            self.process_messages()?;
            self.execute_agents()?;
            self.current_tick += 1;
        }
        Ok(())
    }

    fn process_messages(&mut self) -> Result<()> {
        let messages = std::mem::take(&mut self.message_queue);
        for message in messages {
            if let Some(agent) = self.agents.get_mut(&message.receiver) {
                agent.handle_message(&message);
            }
        }
        Ok(())
    }

    fn execute_agents(&mut self) -> Result<()> {
        for (_, agent) in self.agents.iter_mut() {
            agent.run(self)?;
        }
        Ok(())
    }

    pub fn get_current_agent_id(&self) -> usize {
        // This should be called from within an agent's context
        // In real implementation, track current executing agent
        0 // Simplified for example
    }
}
