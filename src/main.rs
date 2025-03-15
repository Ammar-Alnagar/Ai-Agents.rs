mod agents;
mod behaviors;
mod environment;
mod message;

use agents::{Agent, AgentType};
use environment::Environment;

fn main() -> anyhow::Result<()> {
    let mut env = Environment::new();
    
    // Create different agent types
    let basic_agent = Agent::new(AgentType::Basic);
    let learning_agent = Agent::new(AgentType::Learning);
    
    env.add_agent(basic_agent);
    env.add_agent(learning_agent);
    
    // Run simulation for 5 ticks
    env.run(5)?;
    
    Ok(())
}
