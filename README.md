# Multi-Agent AI System in Rust

## Overview
A modular multi-agent system implementation in Rust demonstrating:
- Agent communication via message passing
- Different agent behavior implementations
- Centralized environment management
- Extensible architecture

## Features
- Multiple agent types with different behaviors
- Message queue system for inter-agent communication
- Tick-based simulation loop
- Error handling with anyhow
- Easy extension with new agent types

## Getting Started
1. Clone the repository
2. Build with `cargo build`
3. Run with `cargo run`

## Project Structure
- **agents/**: Contains different agent implementations
- **behaviors/**: Defines agent behavior traits
- **environment.rs**: Manages simulation environment and message routing
- **message.rs**: Message struct and utilities

## Adding New Agents
1. Create new file in `agents/`
2. Implement `AgentBehavior` trait
3. Add new variant to `AgentType` enum
4. Update `Agent::new()` constructor


## Example Output
```
[LearningAgent] Learning: Hello from BasicAgent #0
[BasicAgent] Received: Learned: Hello from BasicAgent #0
```

## License
MIT License - see LICENSE file for details
```
