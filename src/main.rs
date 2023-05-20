use std::fs::File;
use std::io::{self, BufRead};
use crossterm::terminal::{disable_raw_mode};
use dirs::home_dir;
use tui::backend::{CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::Terminal;
use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen,LeaveAlternateScreen}
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    // create a terminal backend
    let mut stdout = io::stdout();

    // set up terminal
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // let history_output = Command::new("history")
    // .output()
    // .expect("failed to get history");
    let history = read_history()
            .iter() 
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect::<Vec<String>>();

    let mut buffer = String::new();
    loop {

        let input_paragraph = Paragraph::new(format!("{}{}","~>",buffer.as_str()))
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::NONE));
        let history_items = history
            .iter()
            .filter(|cmd| cmd.to_lowercase().contains(&buffer.to_lowercase()))
            .map(|cmd| ListItem::new(cmd.as_str()))
            .collect::<Vec<ListItem>>();
        let history_list = List::new(history_items)
            .block(Block::default().borders(Borders::ALL).title("History"));
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Percentage(90)].as_ref())
            .split(terminal.size()?);
        terminal.draw(|f| {
            f.render_widget(input_paragraph, layout[0]);
            f.render_widget(history_list, layout[1]);
        })?;
        if let Event::Key(key_event) = read().unwrap() {
    if let (KeyCode::Char(c), KeyModifiers::CONTROL) = (key_event.code, key_event.modifiers) {
                if c == 'c' {
                    break;
                }
            }
            if let KeyCode::Char(c) = key_event.code {
                buffer.push(c);
            }
            if KeyCode::Backspace == key_event.code{
                buffer.pop();
            }
        }
    }
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    return Ok(());
}


fn read_history() -> Vec<String> {
    let mut history: Vec<String> = vec!();
    let history_file_path = home_dir().expect("Failed to get home dir!").join(".zsh_history");
    let file = File::open(history_file_path).expect("Couldn't open history");
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let cmd = line.split(";").last().expect(
                    format!("Line '{}' didn't contain command", line).as_str()
                );
                history.push(cmd.to_string());
            }
            Err(err) => {
                eprintln!("Error reading history at line: {}", err);
            }
        }
    }
    return history;
}