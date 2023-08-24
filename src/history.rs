use dirs::home_dir;
use std::fs::File;
use std::io::{self, BufRead};

pub fn read_history() -> Vec<String> {
    return read_history_file()
        .iter()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect::<Vec<String>>();
}

fn read_history_file() -> Vec<String> {
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
