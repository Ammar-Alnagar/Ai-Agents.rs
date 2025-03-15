use crate::behaviors::AgentBehavior;
use crate::environment::Environment;
use crate::message::Message;
use anyhow::Result;

#[derive(Default)]
pub struct LearningAgent {
    knowledge: Vec<String>,
}

impl AgentBehavior for LearningAgent {
    fn run(&mut self, env: &mut Environment) {
        if !self.knowledge.is_empty() {
            let msg = Message::new(
                env.get_current_agent_id(),
                0,
                format!("Learned: {}", self.knowledge.last().unwrap()),
            );
            env.send_message(msg).unwrap();
        }
    }

    fn handle_message(&mut self, message: &Message) {
        println!("[LearningAgent] Learning: {}", message.content);
        self.knowledge.push(message.content.clone());
    }
}