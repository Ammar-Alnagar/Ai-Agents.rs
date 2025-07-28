use crate::agents::basic_agent;
use crate::agents::learning_agent;
use crate::agents::researcher_agent;
use crate::agents::writer_agent;
use crate::behaviors::behavior_trait::AgentBehavior;
use crate::environment::Environment;
use crate::message::Message;

pub enum AgentType {
    Basic,
    Learning,
    Researcher,
    Writer,
}

pub struct Agent {
    pub id: Option<usize>,
    pub behavior: Box<dyn AgentBehavior>,
}

impl Agent {
    pub fn new(agent_type: AgentType) -> Self {
        let behavior: Box<dyn AgentBehavior> = match agent_type {
            AgentType::Basic => Box::new(basic_agent::BasicAgent::default()),
            AgentType::Learning => Box::new(learning_agent::LearningAgent::default()),
            AgentType::Researcher => Box::new(researcher_agent::ResearcherAgent::default()),
            AgentType::Writer => Box::new(writer_agent::WriterAgent::default()),
        };

        Agent { id: None, behavior }
    }

    pub fn run(&mut self, env: &mut Environment) -> Result<(), anyhow::Error> {
        let id = self.id.clone();
        self.behavior.run(id, env)
    }

    pub fn handle_message(&mut self, message: &Message) {
        let id = self.id.clone();
        self.behavior.handle_message(id, message)
    }
}
