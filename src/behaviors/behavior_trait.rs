use crate::environment::Environment;
use crate::message::Message;
use anyhow::Result;

pub trait AgentBehavior {
    fn run(&mut self, env: &mut Environment);
    fn handle_message(&mut self, message: &Message);
}
