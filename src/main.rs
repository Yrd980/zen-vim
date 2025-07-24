use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod app;
mod config;
mod core;
mod ui;
mod modes;
mod picker;

use app::App;

/// Zen-Vim: Minimalist Vim-like editor inspired by Neovim + Snacks
#[derive(Parser, Debug)]
#[command(name = "zen-vim")]
#[command(about = "Minimalist Vim-like terminal editor")]
#[command(version)]
struct Args {
    /// Files to open
    files: Vec<PathBuf>,
    
    /// Show dashboard even with files
    #[arg(short = 'D', long)]
    dashboard: bool,
    
    /// Config directory
    #[arg(short, long)]
    config: Option<PathBuf>,
    
    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Initialize logging
    if args.debug {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    }
    
    // Create and run the application
    let mut app = App::new(args.files, args.config)?;
    
    if args.dashboard || app.should_show_dashboard() {
        app.show_dashboard().await?;
    }
    
    app.run().await?;
    
    Ok(())
} 