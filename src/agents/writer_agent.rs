use crate::behaviors::behavior_trait::AgentBehavior;
use crate::environment::Environment;
use crate::message::Message;
use anyhow::Result;

#[derive(Default)]
pub struct WriterAgent;

impl AgentBehavior for WriterAgent {
    fn run(&mut self, _id: Option<usize>, _env: &mut Environment) -> Result<()> {
        // The writer agent will wait for messages from the researcher
        Ok(())
    }

    fn handle_message(&mut self, _id: Option<usize>, message: &Message) {
        println!("[WriterAgent] Received: {}", message.content);
    }
}
