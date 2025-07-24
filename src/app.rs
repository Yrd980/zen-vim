use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use std::path::PathBuf;
use tokio::time::Duration;

use crate::config::Config;
use crate::core::{BufferManager, Buffer};
use crate::modes::{Mode, ModeManager};
use crate::ui::{UI, Dashboard};
use crate::picker::Picker;

pub struct App {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    config: Config,
    buffer_manager: BufferManager,
    mode_manager: ModeManager,
    ui: UI,
    picker: Option<Picker>,
    dashboard: Option<Dashboard>,
    should_quit: bool,
}

impl App {
    pub fn new(files: Vec<PathBuf>, config_path: Option<PathBuf>) -> Result<Self> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        // Load configuration
        let config = Config::load(config_path)?;
        
        // Initialize components
        let mut buffer_manager = BufferManager::new();
        
        // Open files if provided
        for file in files {
            buffer_manager.open_file(file)?;
        }
        
        let mode_manager = ModeManager::new();
        let ui = UI::new(&config);
        
        Ok(Self {
            terminal,
            config,
            buffer_manager,
            mode_manager,
            ui,
            picker: None,
            dashboard: None,
            should_quit: false,
        })
    }
    
    pub fn should_show_dashboard(&self) -> bool {
        self.buffer_manager.is_empty()
    }
    
    pub async fn show_dashboard(&mut self) -> Result<()> {
        self.dashboard = Some(Dashboard::new(&self.config));
        Ok(())
    }
    
    pub async fn run(&mut self) -> Result<()> {
        loop {
            // Draw UI
            self.terminal.draw(|frame| {
                if let Some(ref dashboard) = self.dashboard {
                    dashboard.render(frame, frame.size());
                } else if let Some(ref mut picker) = self.picker {
                    picker.render(frame, frame.size());
                } else {
                    self.ui.render(
                        frame,
                        &self.buffer_manager,
                        &self.mode_manager,
                        frame.size(),
                    );
                }
            })?;
            
            // Handle events
            if self.handle_events().await? {
                break;
            }
            
            if self.should_quit {
                break;
            }
        }
        
        Ok(())
    }
    
    async fn handle_events(&mut self) -> Result<bool> {
        // Check for events without blocking
        if event::poll(Duration::from_millis(0))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        return self.handle_key_event(key).await;
                    }
                }
                Event::Resize(_, _) => {
                    // Handle terminal resize
                }
                _ => {}
            }
        }
        Ok(false)
    }
    
    async fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<bool> {
        // Handle dashboard
        if let Some(ref mut dashboard) = self.dashboard {
            match dashboard.handle_key(key) {
                Some(action) => {
                    match action.as_str() {
                        "quit" => return Ok(true),
                        "files" => {
                            self.dashboard = None;
                            self.show_file_picker().await?;
                        }
                        "grep" => {
                            self.dashboard = None;
                            self.show_grep_picker().await?;
                        }
                        "buffers" => {
                            self.dashboard = None;
                            self.show_buffer_picker().await?;
                        }
                        "resume" => {
                            self.dashboard = None;
                            self.buffer_manager.resume_session()?;
                        }
                        "rename" => {
                            self.dashboard = None;
                            self.buffer_manager.rename_current_file()?;
                        }
                        _ => {}
                    }
                }
                None => {
                    // Close dashboard on any other key
                    self.dashboard = None;
                }
            }
            return Ok(false);
        }
        
        // Handle picker
        if let Some(ref mut picker) = self.picker {
            match picker.handle_key(key).await? {
                Some(result) => {
                    self.picker = None;
                    if let Some(path) = result.selected_file {
                        self.buffer_manager.open_file(path)?;
                    }
                }
                None => {
                    self.picker = None;
                }
            }
            return Ok(false);
        }
        
        // Handle normal editor keys
        match key.code {
            KeyCode::Char('q') if self.mode_manager.current_mode() == Mode::Normal => {
                return Ok(true); // Quit
            }
            KeyCode::Char(' ') if self.mode_manager.current_mode() == Mode::Normal => {
                // Leader key - only in normal mode
                return self.handle_leader_key().await;
            }
            _ => {
                // Pass to mode manager (handles insert, visual, command modes)
                self.mode_manager.handle_key(key, &mut self.buffer_manager)?;
            }
        }
        
        Ok(false)
    }
    
    async fn handle_leader_key(&mut self) -> Result<bool> {
        // Wait for next key within timeout
        if event::poll(Duration::from_millis(1000))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('f') => {
                        // Find files
                        self.show_file_picker().await?;
                    }
                    KeyCode::Char('/') => {
                        // Grep text
                        self.show_grep_picker().await?;
                    }
                    KeyCode::Char('b') => {
                        // Buffers
                        self.show_buffer_picker().await?;
                    }
                    KeyCode::Char('s') => {
                        // Resume session
                        self.buffer_manager.resume_session()?;
                    }
                    KeyCode::Char('r') => {
                        // Rename file
                        self.buffer_manager.rename_current_file()?;
                    }
                    KeyCode::Char('d') => {
                        // Show dashboard
                        self.dashboard = Some(Dashboard::new(&self.config));
                    }
                    KeyCode::Char('q') => {
                        return Ok(true); // Quit
                    }
                    _ => {}
                }
            }
        }
        Ok(false)
    }
    

    
    async fn show_file_picker(&mut self) -> Result<()> {
        self.picker = Some(Picker::new_file_picker(&self.config).await?);
        Ok(())
    }
    
    async fn show_grep_picker(&mut self) -> Result<()> {
        self.picker = Some(Picker::new_grep_picker(&self.config).await?);
        Ok(())
    }
    
    async fn show_buffer_picker(&mut self) -> Result<()> {
        self.picker = Some(Picker::new_buffer_picker(&self.config, &self.buffer_manager).await?);
        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
    }
} 