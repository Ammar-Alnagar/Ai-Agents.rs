use crate::behaviors::behavior_trait::AgentBehavior;
use crate::environment::Environment;
use crate::message::Message;
use anyhow::Result;

#[derive(Default)]
pub struct LearningAgent {
    knowledge: Vec<String>,
}

impl AgentBehavior for LearningAgent {
    fn run(&mut self, id: Option<usize>, env: &mut Environment) -> Result<()> {
        if !self.knowledge.is_empty() {
            let msg = Message::new(
                id.unwrap(),
                0,
                format!("Learned: {}", self.knowledge.last().unwrap()),
            );
            env.send_message(msg)?;
        }
        Ok(())
    }

    fn handle_message(&mut self, _id: Option<usize>, message: &Message) {
        println!("[LearningAgent] Learning: {}", message.content);
        self.knowledge.push(message.content.clone());
    }
}