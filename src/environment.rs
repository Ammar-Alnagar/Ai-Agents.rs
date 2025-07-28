use crate::agents::Agent;
use crate::message::Message;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

pub struct Environment {
    pub agents: HashMap<usize, Agent>,
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
        agent.id = Some(agent_id);
        self.agents.insert(agent_id, agent);
        self.next_agent_id += 1;
    }

    pub fn send_message(&mut self, message: Message) -> Result<()> {
        if !self.agents.contains_key(&message.receiver_id) {
            return Err(anyhow!("Invalid receiver ID: {}", message.receiver_id));
        }
        self.message_queue.push(message);
        Ok(())
    }

    pub fn run_tick(&mut self) -> Result<Vec<String>> {
        self.process_messages()?;
        self.execute_agents()?;
        self.current_tick += 1;
        let messages = self
            .message_queue
            .iter()
            .map(|m| m.content.clone())
            .collect();
        self.message_queue.clear();
        Ok(messages)
    }

    fn process_messages(&mut self) -> Result<()> {
        let messages = std::mem::take(&mut self.message_queue);
        for message in messages {
            if let Some(agent) = self.agents.get_mut(&message.receiver_id) {
                agent.handle_message(&message);
            }
        }
        Ok(())
    }

    fn execute_agents(&mut self) -> Result<()> {
        let agent_ids: Vec<usize> = self.agents.keys().cloned().collect();
        for id in agent_ids {
            if let Some(mut agent) = self.agents.remove(&id) {
                agent.run(self)?;
                self.agents.insert(id, agent);
            }
        }
        Ok(())
    }
}
