mod render;

use crossterm::terminal::disable_raw_mode;
use crossterm::{
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dirs::home_dir;
use std::fs::File;
use std::io::{self, BufRead};
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

    let history = read_history()
        .iter()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect::<Vec<String>>();

    render::render(&mut terminal, &history).expect("crossterm error during render");
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    return Ok(());
}

fn read_history() -> Vec<String> {
    let mut history: Vec<String> = vec![];
    let history_file_path = home_dir()
        .expect("Failed to get home dir!")
        .join(".zsh_history");
    let file = File::open(history_file_path).expect("Couldn't open .zsh_history");
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let cmd = line
                    .split(";")
                    .last()
                    .expect(format!("Line '{}' didn't contain command", line).as_str());
                history.push(cmd.to_string());
            }
            Err(err) => {
                eprintln!("Error reading history at line: {}", err);
            }
        }
    }
    return history;
}
