use std::{
    fs,
    io::{self, Write},
    process::Command,
    thread,
    time::Duration,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse available art pieces by reading "src" directory.
    // Any *.rs file except main.rs and lib.rs is considered an art piece.
    let mut pieces: Vec<String> = fs::read_dir("src")?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "rs" {
                        let stem = path.file_stem()?.to_string_lossy().to_string();
                        if stem != "main" && stem != "lib" {
                            return Some(stem);
                        }
                    }
                }
            }
            None
        })
        .collect();
    pieces.sort();

    if pieces.is_empty() {
        println!("No art pieces found in src/!");
        return Ok(());
    }

    // Set up terminal in raw mode with alternate screen.
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture, Clear(ClearType::All))?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // ListState to keep track of the selected art piece.
    let mut list_state = ListState::default();
    list_state.select(Some(0));

    'main_loop: loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            // Build a list of art pieces.
            let items: Vec<ListItem> = pieces
                .iter()
                .map(|p| ListItem::new(p.as_str()))
                .collect();

            let list = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Select art piece (Esc or q to exit)"),
                )
                .highlight_style(Style::default().fg(Color::Yellow))
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, chunks[0], &mut list_state);
        })?;

        // Poll for key events.
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    // Exit the main menu if Esc or 'q' is pressed.
                    KeyCode::Char('q') | KeyCode::Esc => break 'main_loop,
                    KeyCode::Down => {
                        let i = match list_state.selected() {
                            Some(i) if i >= pieces.len() - 1 => i,
                            Some(i) => i + 1,
                            None => 0,
                        };
                        list_state.select(Some(i));
                    }
                    KeyCode::Up => {
                        let i = match list_state.selected() {
                            Some(0) | None => 0,
                            Some(i) => i - 1,
                        };
                        list_state.select(Some(i));
                    }
                    KeyCode::Enter => {
                        if let Some(selected) = list_state.selected() {
                            let piece = &pieces[selected];
                            // Restore terminal before running the external command.
                            disable_raw_mode()?;
                            execute!(
                                terminal.backend_mut(),
                                LeaveAlternateScreen,
                                DisableMouseCapture
                            )?;
                            terminal.show_cursor()?;

                            println!("Running: cargo run --release --bin {}", piece);
                            let status = Command::new("cargo")
                                .args(&["run", "--release", "--bin", piece])
                                .status()?;
                            println!("Process exited with status: {}\n", status);

                            // Flush any stray events.
                            while event::poll(Duration::from_millis(0))? {
                                let _ = event::read();
                            }

                            // Small sleep to help terminal settle.
                            thread::sleep(Duration::from_millis(50));

                            // Reinitialize the terminal.
                            enable_raw_mode()?;
                            let mut stdout = io::stdout();
                            execute!(
                                stdout,
                                EnterAlternateScreen,
                                EnableMouseCapture,
                                Clear(ClearType::All)
                            )?;
                            terminal = Terminal::new(CrosstermBackend::new(stdout))?;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // Restore terminal on exit.
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        LeaveAlternateScreen,
        DisableMouseCapture,
        Clear(ClearType::All)
    )?;
    terminal.show_cursor()?;

    Ok(())
}

