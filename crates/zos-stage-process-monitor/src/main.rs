use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;
use std::time::Duration;
use tmux_interface::{Tmux, ListSessions, ListPanes};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Tmux session name to monitor (optional, will list all if not provided)
    #[arg(short, long)]
    session: Option<String>,
    /// Tmux window name to monitor (optional)
    #[arg(short, long)]
    window: Option<String>,
    /// Tmux pane ID to monitor (optional)
    #[arg(short, long)]
    pane: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Application loop
    let mut app_state = AppState::new();

    loop {
        // Update app state (e.g., fetch tmux info)
        app_state.update_tmux_info(&Args::parse())?; // Pass parsed args directly

        // Draw UI
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            let block = Block::default().title("Process Monitor").borders(Borders::ALL);
            let content = Paragraph::new(app_state.get_display_content()).block(block);
            f.render_widget(content, chunks[0]);
        })?;

        // Event handling
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

struct AppState {
    tmux_info: String,
}

impl AppState {
    fn new() -> AppState {
        AppState {
            tmux_info: "Fetching tmux info...".to_string(),
        }
    }

    fn update_tmux_info(
        &mut self,
        args: &Args,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut info = String::new();

        // List sessions
        match Tmux::with_command(ListSessions::new()).output() {
            Ok(output) => {
                info.push_str("Sessions:\n");
                info.push_str(&String::from_utf8_lossy(&output.0.stdout));
            }
            Err(e) => info.push_str(&format!("Error listing sessions: {}\n", e)),
        }

        // List panes (simplified for now, will be more specific later)
        match Tmux::with_command(ListPanes::new()).output() {
            Ok(output) => {
                info.push_str("\nPanes:\n");
                info.push_str(&String::from_utf8_lossy(&output.0.stdout));
            }
            Err(e) => info.push_str(&format!("Error listing panes: {}\n", e)),
        }

        // TODO: Execute 'ps aux' in a selected pane and capture output
        // This will require more advanced tmux_interface usage (send_keys, capture_pane)
        // and parsing the output.

        self.tmux_info = info;
        Ok(())
    }

    fn get_display_content(&self) -> &str {
        &self.tmux_info
    }
}