mod history;
mod render;

use crossterm::terminal::disable_raw_mode;
use crossterm::{
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    // create a terminal backend
    let mut stdout = io::stdout();

    // set up terminal
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let history = history::read_history();

    render::render(&mut terminal, &history).expect("crossterm error during render");
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    return Ok(());
}
