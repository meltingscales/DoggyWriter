pub struct Editor {
    pub current_line: String,
    pub previous_line: String,
    pub all_lines: Vec<String>,
    pub total_chars: usize,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            current_line: String::new(),
            previous_line: String::new(),
            all_lines: Vec::new(),
            total_chars: 0,
        }
    }

    pub fn insert_char(&mut self, c: char, max_width: usize) {
        self.current_line.push(c);
        self.total_chars += 1;

        // Auto-wrap if we exceed the terminal width
        // Account for borders (2 chars) and padding (2 chars) = 4 chars total
        let usable_width = max_width.saturating_sub(4);
        if self.current_line.len() >= usable_width {
            self.auto_wrap();
        }
    }

    fn auto_wrap(&mut self) {
        // Move current line to previous
        self.previous_line = self.current_line.clone();

        // Add current line to all lines
        self.all_lines.push(self.current_line.clone());

        // Clear current line for new input
        self.current_line.clear();

        // Count the newline character
        self.total_chars += 1;
    }

    pub fn backspace(&mut self) {
        if self.current_line.pop().is_some() {
            self.total_chars = self.total_chars.saturating_sub(1);
        }
    }

    pub fn new_line(&mut self) {
        // Move current line to previous
        self.previous_line = self.current_line.clone();

        // Add current line to all lines
        self.all_lines.push(self.current_line.clone());

        // Clear current line for new input
        self.current_line.clear();

        // Count the newline character
        self.total_chars += 1;
    }

    pub fn line_count(&self) -> usize {
        // All committed lines + current line if it has content
        self.all_lines.len() + if self.current_line.is_empty() { 0 } else { 1 }
    }

    pub fn get_all_content(&self) -> String {
        let mut content = self.all_lines.join("\n");
        if !self.all_lines.is_empty() && !self.current_line.is_empty() {
            content.push('\n');
            content.push_str(&self.current_line);
        } else if !self.current_line.is_empty() {
            content = self.current_line.clone();
        }
        content
    }
}
