mod model;
mod storage;
mod terminal;
mod ui;

use std::{io, process::ExitCode};
use terminal::{Key, Terminal};
use ui::Screen;

enum Mode {
    Normal,
    Add(String),
    Rename(String),
    Help,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("2do: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> io::Result<()> {
    let mut list = storage::load()?;
    let terminal = Terminal::enter()?;
    let mut mode = Mode::Normal;
    let mut pending_delete = false;
    let mut message = String::new();

    loop {
        let (width, height) = terminal.size();
        let screen = match &mode {
            Mode::Normal => Screen::List,
            Mode::Add(value) => Screen::Input {
                label: "Nova tarefa",
                value,
            },
            Mode::Rename(value) => Screen::Input {
                label: "Novo nome",
                value,
            },
            Mode::Help => Screen::Help,
        };
        ui::draw(&list, screen, width, height, &message)?;
        message.clear();
        let key = terminal.read_key()?;

        match &mut mode {
            Mode::Add(value) | Mode::Rename(value) => {
                if edit_input(value, key) {
                    let input = value.trim().to_string();
                    if !input.is_empty() {
                        match mode {
                            Mode::Add(_) => list.add(input),
                            Mode::Rename(_) => {
                                if let Some(task) = list.tasks.get_mut(list.selected) {
                                    task.title = input
                                }
                            }
                            _ => {}
                        }
                        storage::save(&list)?;
                    }
                    mode = Mode::Normal;
                } else if matches!(key, Key::Escape) {
                    mode = Mode::Normal;
                }
            }
            Mode::Help => {
                if matches!(key, Key::Char('?') | Key::Escape | Key::Char('q')) {
                    mode = Mode::Normal;
                }
            }
            Mode::Normal => match key {
                Key::Char('q') => break,
                Key::Char('j') | Key::Down => list.move_selection(1),
                Key::Char('k') | Key::Up => list.move_selection(-1),
                Key::Char('g') => list.selected = 0,
                Key::Char('G') if !list.tasks.is_empty() => list.selected = list.tasks.len() - 1,
                Key::Char('a') => mode = Mode::Add(String::new()),
                Key::Char('r') if !list.tasks.is_empty() => {
                    mode = Mode::Rename(list.tasks[list.selected].title.clone())
                }
                Key::Char('J') => {
                    list.move_task(1);
                    storage::save(&list)?;
                }
                Key::Char('K') => {
                    list.move_task(-1);
                    storage::save(&list)?;
                }
                Key::Char(' ') => {
                    list.toggle_selected();
                    storage::save(&list)?;
                }
                Key::Char('d') if pending_delete => {
                    list.remove_selected();
                    storage::save(&list)?;
                    pending_delete = false;
                    message = "Tarefa apagada".into();
                }
                Key::Char('d') => {
                    pending_delete = true;
                    message = "Pressione d novamente para apagar".into();
                }
                Key::Char('?') => mode = Mode::Help,
                _ => pending_delete = false,
            },
        }
    }
    storage::save(&list)
}

fn edit_input(value: &mut String, key: Key) -> bool {
    match key {
        Key::Enter => true,
        Key::Backspace => {
            value.pop();
            false
        }
        Key::Char(character) if !character.is_control() => {
            value.push(character);
            false
        }
        _ => false,
    }
}
