use std::fs;
use std::io;
use crate::editor::Editor;

pub struct Storage {
    filename: Option<String>,
}

impl Storage {
    pub fn new() -> Self {
        Self { filename: None }
    }

    pub fn set_filename(&mut self, filename: String) {
        self.filename = Some(filename);
    }

    pub fn get_filename(&self) -> Option<&String> {
        self.filename.as_ref()
    }

    pub fn save(&self, editor: &Editor) -> io::Result<()> {
        if let Some(filename) = &self.filename {
            let content = editor.get_all_content();
            fs::write(filename, content)?;
        }
        Ok(())
    }
}
