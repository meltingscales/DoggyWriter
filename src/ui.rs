use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, AppState};

pub fn draw(f: &mut Frame, app: &mut App) {
    match app.state {
        AppState::AskingFilename => draw_filename_prompt(f, app),
        AppState::Writing => draw_editor(f, app),
    }
}

fn draw_filename_prompt(f: &mut Frame, app: &App) {
    let area = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    // Welcome message
    let welcome = Paragraph::new("Welcome to DoggyWriter!")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    f.render_widget(welcome, chunks[0]);

    // Instructions
    let instructions = Paragraph::new("Let's create your writing file.")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);
    f.render_widget(instructions, chunks[1]);

    // Filename input
    let input_text = format!("Filename: {}", app.filename_input);
    let input = Paragraph::new(input_text)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Enter filename"));
    f.render_widget(input, chunks[2]);

    // Show cursor at the end of filename input
    if !app.filename_input.is_empty() {
        let cursor_x = chunks[2].x + 11 + app.filename_input.len() as u16;
        let cursor_y = chunks[2].y + 1;
        f.set_cursor(cursor_x, cursor_y);
    } else {
        let cursor_x = chunks[2].x + 11;
        let cursor_y = chunks[2].y + 1;
        f.set_cursor(cursor_x, cursor_y);
    }
}

fn draw_editor(f: &mut Frame, app: &mut App) {
    let area = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),     // Main content area
            Constraint::Length(3),  // Status bar
        ])
        .split(area);

    // Create the two-line display area
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Previous line
            Constraint::Length(3),  // Current line
            Constraint::Min(0),     // Empty space
        ])
        .split(chunks[0]);

    // Update terminal width in app based on the current line widget width
    app.update_terminal_width(content_chunks[1].width);

    // Previous line
    let previous_line_text = if app.editor.previous_line.is_empty() {
        "(no previous line yet)".to_string()
    } else {
        app.editor.previous_line.clone()
    };

    let previous = Paragraph::new(previous_line_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL).title("Previous"));
    f.render_widget(previous, content_chunks[0]);

    // Current line
    let current_line_text = if app.editor.current_line.is_empty() {
        " ".to_string()
    } else {
        app.editor.current_line.clone()
    };

    let current = Paragraph::new(current_line_text)
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title("Current").style(Style::default().fg(Color::Cyan)));
    f.render_widget(current, content_chunks[1]);

    // Show cursor at the end of current line
    let cursor_x = content_chunks[1].x + 1 + app.editor.current_line.len() as u16;
    let cursor_y = content_chunks[1].y + 1;
    f.set_cursor(cursor_x, cursor_y);

    // Status bar
    let filename = app.storage.get_filename().unwrap_or(&"unknown".to_string()).clone();
    let line_count = app.editor.line_count();
    let char_count = app.editor.total_chars;
    let save_indicator = app.get_save_indicator_text();

    let status_line_1 = Line::from(vec![
        Span::styled(
            format!("{} ", filename),
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("| Lines: {} ", line_count),
            Style::default().fg(Color::Yellow),
        ),
        Span::styled(
            format!("| Chars: {} ", char_count),
            Style::default().fg(Color::Yellow),
        ),
        Span::styled(
            format!("| {}", save_indicator),
            Style::default().fg(Color::Magenta),
        ),
    ]);

    let status_line_2 = Line::from(vec![
        Span::styled(
            "Ctrl+Q: Quit ",
            Style::default().fg(Color::Cyan),
        ),
        Span::styled(
            "| Ctrl+S: Save ",
            Style::default().fg(Color::Cyan),
        ),
        Span::styled(
            "| Enter: New line ",
            Style::default().fg(Color::Cyan),
        ),
        Span::styled(
            "| Backspace: Delete",
            Style::default().fg(Color::Cyan),
        ),
    ]);

    let status = Paragraph::new(vec![status_line_1, status_line_2])
        .block(Block::default().borders(Borders::ALL).title("Status"))
        .wrap(Wrap { trim: false });

    f.render_widget(status, chunks[1]);
}
