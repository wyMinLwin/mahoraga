mod app;
mod commands;
mod config;
mod providers;
mod types;
mod ui;

use std::io;

use anyhow::Result;
use clap::{Parser, Subcommand};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::app::App;

#[derive(Parser)]
#[command(name = "mahoraga")]
#[command(author = "Mahoraga Contributors")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "A terminal-based prompt validation and improvement tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch the TUI application
    Summon,
    /// Update mahoraga to the latest version
    Update,
    /// Uninstall mahoraga (removes binary and config)
    Uninstall,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Summon) | None => {
            run_tui().await?;
        }
        Some(Commands::Update) => {
            commands::run_update().await?;
        }
        Some(Commands::Uninstall) => {
            commands::run_uninstall()?;
        }
    }

    Ok(())
}

async fn run_tui() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create and run app
    let mut app = App::new()?;
    let result = app.run(&mut terminal).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}
