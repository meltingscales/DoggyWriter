# DoggyWriter - Plan & Design Document

## What is DoggyWriter?

Hey Milo! This is a writing app built just for you. Think of it like a special typewriter that helps you focus on *just writing* without getting stuck overthinking.

**The Big Idea:** Sometimes when we write, we keep going back to edit and re-read everything, which stops us from making progress. DoggyWriter solves this by only showing you 2 lines at a time - the line you're currently writing and the previous line. This way, you can keep moving forward!

**How it works (for beginners):**
- **Rust**: The programming language we're using. It's fast and reliable (like a well-trained dog!).
- **ratatui**: A library that lets us make cool text-based interfaces in the terminal (the command line window).
- **sed/ed-like**: These are classic text editors that work with simple commands. DoggyWriter takes inspiration from them but keeps things simple.
- **Auto-save**: Your work saves automatically to a .txt file, so you never lose your writing!

---

## Core Features

### 1. **Minimal Display (2-Line View)**
- Show only the current line being typed
- Show the previous line for context
- This creates a "tunnel vision" effect to prevent analysis paralysis
- **Automatic line wrapping**: Lines automatically wrap based on terminal width (no fixed character limit!)
- Status bar at the bottom showing: filename, line count, character count

### 2. **Simple Controls**
- **Type normally**: Characters appear on the current line
- **Enter**: Create a new line (pushes current line up to "previous" position)
- **Backspace**: Delete characters on current line only
- **Cannot go back**: Once you press Enter, that line is locked (you can't navigate back to edit it)
- **Ctrl+Q**: Quit the application
- **Ctrl+S**: Manual save (though auto-save happens anyway)

### 3. **Auto-Save System**
- On first launch, prompt: "What would you like to name your file?"
- User types a filename (e.g., "my-story")
- App automatically adds `.txt` extension
- Saves to current directory
- Auto-saves every 5 seconds while typing
- Also saves when you quit

### 4. **Session Management**
- First run: Ask for filename
- Subsequent runs: Can either:
  - Continue last file (default)
  - Start new file (option to specify)

---

## Technical Architecture

### Project Structure
```
DoggyWriter/
├── Cargo.toml           # Rust project configuration
├── justfile             # Build automation commands
├── PLAN.md             # This file!
├── README.md           # User-facing documentation
├── src/
│   ├── main.rs         # Entry point, event loop
│   ├── app.rs          # Application state and logic
│   ├── ui.rs           # UI rendering with ratatui
│   ├── editor.rs       # Text editing logic
│   └── storage.rs      # File I/O and auto-save
```

### Key Components

#### 1. **App State** (`app.rs`)
```rust
struct App {
    filename: Option<String>,
    current_line: String,
    previous_line: String,
    all_lines: Vec<String>,
    total_chars: usize,
    is_running: bool,
    last_save_time: Instant,
}
```

#### 2. **Editor Logic** (`editor.rs`)
- Handle character input
- Handle backspace (current line only)
- Handle Enter (commit line, move to history)
- Prevent upward navigation

#### 3. **UI Rendering** (`ui.rs`)
- Create 2-line display area
- Status bar with:
  - Filename
  - Line count
  - Character count
  - Last saved indicator
- Welcome/filename prompt screen

#### 4. **Storage** (`storage.rs`)
- Prompt for filename
- Auto-save every 5 seconds
- Save on quit
- Load existing file if continuing session

---

## Implementation Plan

### Phase 1: Basic Setup
1. Initialize Cargo project with ratatui dependency
2. Set up basic event loop (keyboard input handling)
3. Create simple UI with 2-line display
4. Implement basic typing (characters appear on screen)

### Phase 2: Editor Mechanics
1. Implement Enter key (line advancement)
2. Implement Backspace (current line only)
3. Prevent cursor from going to previous lines
4. Track all lines in memory

### Phase 3: File I/O
1. Create filename prompt screen
2. Implement save function
3. Add auto-save timer (every 5 seconds)
4. Add save-on-quit

### Phase 4: Polish
1. Add status bar with stats
2. Add visual feedback (e.g., "Saved!" indicator)
3. Add keyboard shortcuts help text
4. Error handling (file permissions, disk full, etc.)

### Phase 5: Build System
1. Create `justfile` with targets:
   - `just build` - Compile the project
   - `just run` - Run the app
   - `just release` - Build optimized version
   - `just publish` - Publish to crates.io
   - `just install` - Install locally

---

## Justfile Targets

```just
# Default recipe (run when you type 'just')
default: run

# Build the project
build:
    cargo build

# Run the application
run:
    cargo run

# Build optimized release version
release:
    cargo build --release

# Run tests
test:
    cargo test

# Publish to crates.io
publish:
    cargo publish

# Install locally
install:
    cargo install --path .

# Clean build artifacts
clean:
    cargo clean
```

---

## Dependencies (Cargo.toml)

```toml
[dependencies]
ratatui = "0.26"
crossterm = "0.27"  # For terminal input/output handling
```

---

## User Experience Flow

### First Launch
```
┌─────────────────────────────────────────┐
│        Welcome to DoggyWriter!          │
│                                         │
│  Let's create your writing file.       │
│                                         │
│  Filename: my-story█                    │
│                                         │
│  (Press Enter when ready)               │
└─────────────────────────────────────────┘
```

### Writing Session
```
┌─────────────────────────────────────────┐
│ Previous: The quick brown fox jumped    │
│ Current: over the lazy dog█             │
├─────────────────────────────────────────┤
│ my-story.txt | Lines: 42 | Chars: 1,234 │
│ Ctrl+Q: Quit | Auto-saved 2s ago        │
└─────────────────────────────────────────┘
```

---

## Why This Design?

1. **Two-line view**: Provides just enough context without overwhelming
2. **No backward editing**: Forces forward momentum (key to beating analysis paralysis!)
3. **Auto-save**: No fear of losing work
4. **Minimal UI**: Less distraction = more writing
5. **Simple controls**: Easy to learn, hard to mess up

---

## Future Enhancement Ideas

(Not in initial version, but cool ideas for later!)

- Word count goals ("Write 500 words!")
- Daily streak tracking
- Optional "peek mode" to review last 10 lines
- Themes/color schemes
- Export to Markdown
- Writing statistics (words per minute, etc.)

---

## Getting Started (For Milo!)

Once we build this, here's how you'll use it:

1. Open your terminal
2. Type: `just run`
3. Enter a filename for your story
4. Start writing!
5. Press Ctrl+Q when done

The app saves automatically, so just focus on writing. The two-line view keeps you in the flow without getting distracted by editing what you already wrote.

Happy writing! 🐕✍️
