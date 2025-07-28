use crate::behaviors::behavior_trait::AgentBehavior;
use crate::environment::Environment;
use crate::message::Message;
use anyhow::Result;

#[derive(Default)]
pub struct BasicAgent {
    message_count: u32,
}

impl AgentBehavior for BasicAgent {
    fn run(&mut self, id: Option<usize>, env: &mut Environment) -> Result<()> {
        if self.message_count % 3 == 0 {
            let msg = Message::new(
                id.unwrap(),
                1,
                format!("Hello from BasicAgent #{}", self.message_count),
            );
            env.send_message(msg)?;
        }
        self.message_count += 1;
        Ok(())
    }

    fn handle_message(&mut self, _id: Option<usize>, message: &Message) {
        println!("[BasicAgent] Received: {}", message.content);
    }
}