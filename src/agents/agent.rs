use crate::behaviors::AgentBehavior;
use crate::message::Message;

pub enum AgentType {
    Basic,
    Learning,
}

pub struct Agent {
    id: usize,
    behavior: Box<dyn AgentBehavior>,
}

impl Agent {
    pub fn new(agent_type: AgentType) -> Self {
        let behavior: Box<dyn AgentBehavior> = match agent_type {
            AgentType::Basic => Box::new(basic_agent::BasicAgent::default()),
            AgentType::Learning => Box::new(learning_agent::LearningAgent::default()),
        };
        
        Agent {
            id: 0, // Will be set by environment
            behavior,
        }
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    pub fn run(&mut self, env: &mut Environment) {
        self.behavior.run(env)
    }

    pub fn handle_message(&mut self, message: &Message) {
        self.behavior.handle_message(message)
    }
}
