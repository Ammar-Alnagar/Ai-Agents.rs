mod agents;
mod behaviors;
mod config;
mod environment;
mod message;
mod tools;

use crate::config::Config;
use agents::{Agent, AgentType};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use environment::Environment;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem},
};
use std::{io, time::Duration};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_file("config.json")?;
    let (tx, mut rx) = mpsc::unbounded_channel();

    let mut env = Environment::new();

    for agent_config in config.agents {
        let agent_type = match agent_config.name.as_str() {
            "researcher_agent" => AgentType::Researcher,
            "writer_agent" => AgentType::Writer,
            _ => AgentType::Basic,
        };
        let agent = Agent::new(agent_type);
        env.add_agent(agent);
    }

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App {
        messages: Vec::new(),
    };

    tokio::spawn(async move {
        loop {
            if let Ok(message) = env.run_tick() {
                for msg in message {
                    tx.send(msg).unwrap();
                }
            }
            tokio::time::sleep(Duration::from_millis(1000)).await;
        }
    });

    run_app(&mut terminal, &mut app, &mut rx).await?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

struct App {
    messages: Vec<String>,
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    rx: &mut mpsc::UnboundedReceiver<String>,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::<B>(f, app))?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }

        if let Ok(message) = rx.try_recv() {
            app.messages.push(message);
        }
    }
}

fn ui<B: Backend>(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .map(|m| ListItem::new(m.as_str()))
        .collect();
    let messages = List::new(messages)
        .block(Block::default().borders(Borders::ALL).title("Messages"))
        .start_corner(Corner::TopLeft);
    f.render_widget(messages, chunks[0]);
}
