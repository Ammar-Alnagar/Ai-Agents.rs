use crate::behaviors::behavior_trait::AgentBehavior;
use crate::environment::Environment;
use crate::message::Message;
use crate::tools::web_search;
use anyhow::Result;

#[derive(Default)]
pub struct ResearcherAgent;

impl AgentBehavior for ResearcherAgent {
    fn run(&mut self, id: Option<usize>, env: &mut Environment) -> Result<()> {
        let query = "multi-agent systems";
        let search_results = web_search::search(query);
        let msg = Message::new(id.unwrap(), 1, search_results);
        env.send_message(msg)?;
        Ok(())
    }

    fn handle_message(&mut self, _id: Option<usize>, message: &Message) {
        println!("[ResearcherAgent] Received: {}", message.content);
    }
}
