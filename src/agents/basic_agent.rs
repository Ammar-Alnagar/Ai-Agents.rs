use crate::behaviors::AgentBehavior;
use crate::environment::Environment;
use crate::message::Message;
use anyhow::Result;

#[derive(Default)]
pub struct BasicAgent {
    message_count: u32,
}

impl AgentBehavior for BasicAgent {
    fn run(&mut self, env: &mut Environment) {
        if self.message_count % 3 == 0 {
            let msg = Message::new(
                env.get_current_agent_id(),
                1,
                format!("Hello from BasicAgent #{}", self.message_count),
            );
            env.send_message(msg).unwrap();
        }
        self.message_count += 1;
    }

    fn handle_message(&mut self, message: &Message) {
        println!("[BasicAgent] Received: {}", message.content);
    }
}