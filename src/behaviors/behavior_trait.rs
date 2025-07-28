use crate::environment::Environment;
use crate::message::Message;
use anyhow::Result;

pub trait AgentBehavior: Send + Sync {
    fn run(&mut self, id: Option<usize>, env: &mut Environment) -> Result<()>;
    fn handle_message(&mut self, id: Option<usize>, message: &Message);
}
