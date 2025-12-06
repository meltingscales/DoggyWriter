use crossterm::event::{KeyCode, KeyEvent};
use std::io;
use std::time::{Duration, Instant};

use crate::editor::Editor;
use crate::storage::Storage;

pub enum AppState {
    AskingFilename,
    Writing,
}

pub struct App {
    pub state: AppState,
    pub editor: Editor,
    pub storage: Storage,
    pub filename_input: String,
    running: bool,
    last_save_time: Instant,
    pub last_save_indicator: Option<Instant>,
    pub terminal_width: u16,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::AskingFilename,
            editor: Editor::new(),
            storage: Storage::new(),
            filename_input: String::new(),
            running: true,
            last_save_time: Instant::now(),
            last_save_indicator: None,
            terminal_width: 80, // Default, will be updated by UI
        }
    }

    pub fn update_terminal_width(&mut self, width: u16) {
        self.terminal_width = width;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> io::Result<()> {
        match self.state {
            AppState::AskingFilename => {
                match key.code {
                    KeyCode::Char(c) => {
                        self.filename_input.push(c);
                    }
                    KeyCode::Backspace => {
                        self.filename_input.pop();
                    }
                    KeyCode::Enter => {
                        if !self.filename_input.is_empty() {
                            let filename = if self.filename_input.ends_with(".txt") {
                                self.filename_input.clone()
                            } else {
                                format!("{}.txt", self.filename_input)
                            };
                            self.storage.set_filename(filename);
                            self.state = AppState::Writing;
                        }
                    }
                    _ => {}
                }
            }
            AppState::Writing => {
                match key.code {
                    KeyCode::Char(c) => {
                        self.editor.insert_char(c, self.terminal_width as usize);
                    }
                    KeyCode::Backspace => {
                        self.editor.backspace();
                    }
                    KeyCode::Enter => {
                        self.editor.new_line();
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn save(&mut self) -> io::Result<()> {
        if let AppState::Writing = self.state {
            self.storage.save(&self.editor)?;
            self.last_save_time = Instant::now();
            self.last_save_indicator = Some(Instant::now());
        }
        Ok(())
    }

    pub fn auto_save(&mut self) -> io::Result<()> {
        if let AppState::Writing = self.state {
            if self.last_save_time.elapsed() >= Duration::from_secs(5) {
                self.save()?;
            }
        }
        Ok(())
    }

    pub fn get_save_indicator_text(&self) -> String {
        if let Some(save_time) = self.last_save_indicator {
            let elapsed = save_time.elapsed();
            if elapsed < Duration::from_secs(3) {
                return "Saved!".to_string();
            } else {
                let secs = elapsed.as_secs();
                return format!("Auto-saved {}s ago", secs);
            }
        }
        "Not saved yet".to_string()
    }
}
