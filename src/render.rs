use std::io::Stdout;

use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    Result,
};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::Terminal;

pub fn render(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    history: &Vec<String>,
) -> Result<()> {
    let mut buffer = String::new();
    loop {
        let input_paragraph = Paragraph::new(format!("{}{}", "~>", buffer.as_str()))
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::NONE));
        let history_items = history
            .iter()
            .filter(|cmd| cmd.to_lowercase().contains(&buffer.to_lowercase()))
            .map(|cmd| ListItem::new(cmd.as_str()))
            .collect::<Vec<ListItem>>();
        let history_list =
            List::new(history_items).block(Block::default().borders(Borders::ALL).title("History"));
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Percentage(90)].as_ref())
            .split(terminal.size()?);
        terminal.draw(|f| {
            f.render_widget(input_paragraph, layout[0]);
            f.render_widget(history_list, layout[1]);
        })?;
        if let Event::Key(key_event) = read().unwrap() {
            if let (KeyCode::Char(c), KeyModifiers::CONTROL) = (key_event.code, key_event.modifiers)
            {
                if c == 'c' {
                    break;
                }
            }
            if let KeyCode::Char(c) = key_event.code {
                buffer.push(c);
            }
            if KeyCode::Backspace == key_event.code {
                buffer.pop();
            }
        }
    }
    return Ok(());
}
