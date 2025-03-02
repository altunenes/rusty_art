use std::{
    collections::HashSet,
    env,
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
    // Process command-line arguments.
    // Default mode uses cargo run error output.
    // If "--src" is provided, switch to scanning the "src" directory.
    let args: Vec<String> = env::args().collect();
    let mut mode = "cargo_run"; // default mode
    let mut src_dir = "src".to_string();
    for arg in args.iter().skip(1) {
        if arg == "--src" {
            mode = "src";
            src_dir = "src".to_string();
        }
    }

    // Get list of available binaries.
    let mut pieces: Vec<String> = Vec::new();
    if mode == "src" {
        // Scan the "src" directory for .rs files, excluding main.rs and lib.rs.
        pieces = fs::read_dir(&src_dir)?
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
    } else {
        // Run `cargo run` and parse the error output to extract available binaries.
        let output = Command::new("cargo")
            .arg("run")
            .output()?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        if let Some(index) = stderr.find("available binaries:") {
            let binaries_str = &stderr[index..];
            if let Some(colon_index) = binaries_str.find(':') {
                let bin_list = &binaries_str[colon_index + 1..];
                pieces = bin_list
                    .split(|c| c == ',' || c == '\n')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
        }
    }

    pieces.sort();
    if pieces.is_empty() {
        println!("No art pieces found using mode {}!", mode);
        return Ok(());
    }

    // Load run history from file (run_history.txt) into a HashSet.
    let history_path = "run_history.txt";
    let mut run_history: HashSet<String> = HashSet::new();
    if let Ok(contents) = fs::read_to_string(history_path) {
        for line in contents.lines() {
            if !line.trim().is_empty() {
                run_history.insert(line.trim().to_string());
            }
        }
    }

    // Set up terminal in raw mode with an alternate screen.
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture, Clear(ClearType::All))?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // ListState to track the selected art piece.
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
            // If an art piece is in run_history, style it with blue.
            let items: Vec<ListItem> = pieces
                .iter()
                .map(|p| {
                    let mut item = ListItem::new(p.as_str());
                    if run_history.contains(p) {
                        item = item.style(Style::default().fg(Color::Blue));
                    }
                    item
                })
                .collect();

            // Title now includes the number of pieces found.
            let title = format!(
                "Select art piece ({} pieces found, Esc or q to exit)",
                pieces.len()
            );

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(title))
                .highlight_style(Style::default().fg(Color::Yellow))
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, chunks[0], &mut list_state);
        })?;

        // Poll for key events.
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    // Exit on Esc or 'q'.
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

                            // Update run history and save to file.
                            if run_history.insert(piece.clone()) {
                                let history_data = run_history
                                    .iter()
                                    .cloned()
                                    .collect::<Vec<_>>()
                                    .join("\n");
                                fs::write(history_path, history_data)?;
                            }

                            // Flush stray events.
                            while event::poll(Duration::from_millis(0))? {
                                let _ = event::read();
                            }

                            // Small sleep to let the terminal settle.
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

