# DoggyWriter

[![Crates.io](https://img.shields.io/crates/v/doggywriter.svg)](https://crates.io/crates/doggywriter)
[![License](https://img.shields.io/crates/l/doggywriter.svg)](https://github.com/meltingscales/DoggyWriter#license)

A minimal writing app that helps you avoid analysis paralysis by keeping you focused on writing forward.

## What is DoggyWriter?

DoggyWriter is a terminal-based writing tool that shows you only 2 lines at a time: the line you're currently typing and the previous line. This "tunnel vision" approach helps you stay in the flow and avoid getting stuck editing what you've already written.

## Features

- **2-Line View**: See only your current line and the previous line
- **Forward-Only Writing**: Once you press Enter, that line is locked (no going back to edit)
- **Auto-Save**: Your work saves automatically every 5 seconds
- **Simple Controls**: Just type, press Enter for new lines, and Backspace to fix the current line
- **Distraction-Free**: Minimal UI keeps you focused on writing

## Installation

### Install from Crates.io (Recommended)

The easiest way to install DoggyWriter:

```bash
cargo install doggywriter
```

This requires [Rust](https://rustup.rs/) to be installed.

### Install from Source

```bash
# Clone the repository
git clone https://github.com/meltingscales/DoggyWriter
cd DoggyWriter

# Install locally (puts binary in ~/.cargo/bin/)
just install
# or
cargo install --path .
```

## Usage

### Running DoggyWriter

```bash
# If you have 'just' installed:
just run

# Or with cargo:
cargo run
```

### First Launch

1. You'll be prompted to enter a filename for your writing
2. Type a name (e.g., `my-story`) and press Enter
3. The app automatically adds `.txt` extension
4. Start writing!

### Controls

- **Type normally**: Characters appear on the current line
- **Enter**: Create a new line (locks the previous line)
- **Backspace**: Delete characters from the current line only
- **Ctrl+S**: Manually save (though it auto-saves every 5 seconds)
- **Ctrl+Q**: Quit the application (auto-saves on exit)

### The Interface

```
┌─────────────────────────────────────────┐
│ Previous: The quick brown fox jumped    │
│ Current: over the lazy dog█             │
├─────────────────────────────────────────┤
│ my-story.txt | Lines: 42 | Chars: 1,234 │
│ Ctrl+Q: Quit | Auto-saved 2s ago        │
└─────────────────────────────────────────┘
```

## Development

### Building

```bash
# Debug build
just build

# Release build (optimized)
just release
```

### Running Tests

```bash
just test
```

### Code Quality

```bash
# Format code
just fmt

# Run linter
just lint

# Check code without building
just check
```

## Publishing

```bash
# Publish to crates.io (make sure you're logged in)
just publish
```

## Why DoggyWriter?

Sometimes the best way to write is to just write. By removing the ability to constantly edit and re-read what you've written, DoggyWriter keeps you moving forward and helps you:

- **Beat analysis paralysis**: Can't overthink what you can't see
- **Maintain flow**: Keep writing without breaking your creative momentum
- **Focus on drafting**: Save the editing for later (that's what second drafts are for!)

Perfect for:
- First drafts
- Freewriting sessions
- Brainstorming
- Journal entries
- Stream-of-consciousness writing

## License

MIT OR Apache-2.0

## Contributing

Contributions welcome! Feel free to open issues or pull requests.
