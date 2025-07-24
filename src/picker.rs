use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ignore::WalkBuilder;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
// use regex::Regex;
use std::path::PathBuf;
use std::process::Command;

use crate::config::Config;
use crate::core::BufferManager;

pub struct PickerResult {
    pub selected_file: Option<PathBuf>,
    pub selected_buffer_id: Option<usize>,
}

pub enum PickerType {
    Files,
    Grep(String),
    Buffers,
}

pub struct Picker {
    picker_type: PickerType,
    items: Vec<PickerItem>,
    filtered_items: Vec<usize>,
    list_state: ListState,
    input: String,
    show_preview: bool,
}

#[derive(Debug, Clone)]
struct PickerItem {
    display: String,
    path: Option<PathBuf>,
    buffer_id: Option<usize>,
    line_number: Option<usize>,
    match_text: Option<String>,
}

impl Picker {
    pub async fn new_file_picker(config: &Config) -> Result<Self> {
        let mut items = Vec::new();
        let current_dir = std::env::current_dir()?;
        
        let walker = WalkBuilder::new(&current_dir)
            .hidden(false)
            .git_ignore(true)
            .build();
        
        for entry in walker {
            if let Ok(entry) = entry {
                if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                    let path = entry.path().to_path_buf();
                    let display = path
                        .strip_prefix(&current_dir)
                        .unwrap_or(&path)
                        .to_string_lossy()
                        .to_string();
                    
                    // Filter by ignore patterns
                    if !config.picker.file_ignore_patterns.iter().any(|pattern| {
                        display.contains(pattern) || path.to_string_lossy().contains(pattern)
                    }) {
                        items.push(PickerItem {
                            display,
                            path: Some(path),
                            buffer_id: None,
                            line_number: None,
                            match_text: None,
                        });
                    }
                }
            }
        }
        
        // Limit results
        items.truncate(config.picker.max_results);
        let filtered_items: Vec<usize> = (0..items.len()).collect();
        
        let mut list_state = ListState::default();
        if !items.is_empty() {
            list_state.select(Some(0));
        }
        
        Ok(Self {
            picker_type: PickerType::Files,
            items,
            filtered_items,
            list_state,
            input: String::new(),
            show_preview: config.picker.preview_enabled,
        })
    }
    
    pub async fn new_grep_picker(config: &Config) -> Result<Self> {
        // Start with empty items, will be populated when user types
        Ok(Self {
            picker_type: PickerType::Grep(String::new()),
            items: Vec::new(),
            filtered_items: Vec::new(),
            list_state: ListState::default(),
            input: String::new(),
            show_preview: config.picker.preview_enabled,
        })
    }
    
    pub async fn new_buffer_picker(config: &Config, buffer_manager: &BufferManager) -> Result<Self> {
        let mut items = Vec::new();
        
        for buffer in buffer_manager.list_buffers() {
            let display = if buffer.modified {
                format!("{} [+]", buffer.name)
            } else {
                buffer.name.clone()
            };
            
            items.push(PickerItem {
                display,
                path: buffer.path.clone(),
                buffer_id: Some(buffer.id),
                line_number: None,
                match_text: None,
            });
        }
        
        let filtered_items: Vec<usize> = (0..items.len()).collect();
        
        let mut list_state = ListState::default();
        if !items.is_empty() {
            list_state.select(Some(0));
        }
        
        Ok(Self {
            picker_type: PickerType::Buffers,
            items,
            filtered_items,
            list_state,
            input: String::new(),
            show_preview: config.picker.preview_enabled,
        })
    }
    
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = if self.show_preview {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(area)
        } else {
            std::rc::Rc::from([area].as_slice())
        };
        
        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(chunks[0]);
        
        // Render input box
        let input_block = Block::default()
            .borders(Borders::ALL)
            .title(match &self.picker_type {
                PickerType::Files => "Find Files",
                PickerType::Grep(_) => "Grep",
                PickerType::Buffers => "Buffers",
            });
        
        let input_paragraph = Paragraph::new(self.input.as_str())
            .block(input_block)
            .style(Style::default().fg(Color::White));
        
        frame.render_widget(input_paragraph, left_chunks[0]);
        
        // Set cursor in input box
        frame.set_cursor(
            left_chunks[0].x + self.input.len() as u16 + 1,
            left_chunks[0].y + 1,
        );
        
        // Render list
        let list_items: Vec<ListItem> = self
            .filtered_items
            .iter()
            .map(|&i| {
                let item = &self.items[i];
                let style = Style::default().fg(Color::White);
                ListItem::new(item.display.clone()).style(style)
            })
            .collect();
        
        let list = List::new(list_items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::DarkGray));
        
        frame.render_stateful_widget(list, left_chunks[1], &mut self.list_state);
        
        // Render preview if enabled
        if self.show_preview && chunks.len() > 1 {
            self.render_preview(frame, chunks[1]);
        }
    }
    
    fn render_preview(&self, frame: &mut Frame, area: Rect) {
        let preview_block = Block::default()
            .borders(Borders::ALL)
            .title("Preview");
        
        if let Some(selected_idx) = self.list_state.selected() {
            if let Some(&item_idx) = self.filtered_items.get(selected_idx) {
                if let Some(item) = self.items.get(item_idx) {
                    if let Some(ref path) = item.path {
                        // Try to read file for preview
                        match std::fs::read_to_string(path) {
                            Ok(content) => {
                                let lines: Vec<Line> = content
                                    .lines()
                                    .take(50) // Limit preview lines
                                    .map(|line| Line::from(line.to_string()))
                                    .collect();
                                
                                let preview = Paragraph::new(lines)
                                    .block(preview_block)
                                    .wrap(ratatui::widgets::Wrap { trim: true });
                                
                                frame.render_widget(preview, area);
                                return;
                            }
                            Err(_) => {
                                // File couldn't be read
                            }
                        }
                    }
                }
            }
        }
        
        // Default preview
        let preview = Paragraph::new("No preview available")
            .block(preview_block)
            .style(Style::default().fg(Color::DarkGray));
        
        frame.render_widget(preview, area);
    }
    
    pub async fn handle_key(&mut self, key: KeyEvent) -> Result<Option<PickerResult>> {
        match key.code {
            KeyCode::Esc => {
                return Ok(None); // Cancel picker
            }
            KeyCode::Enter => {
                return Ok(Some(self.select_current()));
            }
            KeyCode::Up => {
                self.move_selection_up();
            }
            KeyCode::Down => {
                self.move_selection_down();
            }
            KeyCode::Char(c) => {
                self.input.push(c);
                self.update_filter().await?;
            }
            KeyCode::Backspace => {
                self.input.pop();
                self.update_filter().await?;
            }
            _ => {}
        }
        
        Ok(None)
    }
    
    fn move_selection_up(&mut self) {
        if !self.filtered_items.is_empty() {
            let selected = self.list_state.selected().unwrap_or(0);
            let new_selected = if selected > 0 {
                selected - 1
            } else {
                self.filtered_items.len() - 1
            };
            self.list_state.select(Some(new_selected));
        }
    }
    
    fn move_selection_down(&mut self) {
        if !self.filtered_items.is_empty() {
            let selected = self.list_state.selected().unwrap_or(0);
            let new_selected = if selected + 1 < self.filtered_items.len() {
                selected + 1
            } else {
                0
            };
            self.list_state.select(Some(new_selected));
        }
    }
    
    fn select_current(&self) -> PickerResult {
        if let Some(selected_idx) = self.list_state.selected() {
            if let Some(&item_idx) = self.filtered_items.get(selected_idx) {
                if let Some(item) = self.items.get(item_idx) {
                    return PickerResult {
                        selected_file: item.path.clone(),
                        selected_buffer_id: item.buffer_id,
                    };
                }
            }
        }
        
        PickerResult {
            selected_file: None,
            selected_buffer_id: None,
        }
    }
    
    async fn update_filter(&mut self) -> Result<()> {
        if self.input.is_empty() {
            // Show all items
            self.filtered_items = (0..self.items.len()).collect();
        } else {
            match &self.picker_type {
                PickerType::Files | PickerType::Buffers => {
                    // Simple substring filtering
                    self.filtered_items = self
                        .items
                        .iter()
                        .enumerate()
                        .filter(|(_, item)| {
                            item.display.to_lowercase().contains(&self.input.to_lowercase())
                        })
                        .map(|(i, _)| i)
                        .collect();
                }
                PickerType::Grep(_) => {
                    // Perform actual grep search
                    self.perform_grep_search().await?;
                }
            }
        }
        
        // Reset selection
        self.list_state.select(if self.filtered_items.is_empty() {
            None
        } else {
            Some(0)
        });
        
        Ok(())
    }
    
    async fn perform_grep_search(&mut self) -> Result<()> {
        if self.input.trim().is_empty() {
            self.items.clear();
            self.filtered_items.clear();
            return Ok(());
        }
        
        self.items.clear();
        
        // Use ripgrep if available, otherwise fall back to grep
        let output = if Command::new("rg").arg("--version").output().is_ok() {
            Command::new("rg")
                .args(&[
                    "--line-number",
                    "--no-heading",
                    "--with-filename",
                    &self.input,
                ])
                .output()
        } else {
            Command::new("grep")
                .args(&["-rn", &self.input, "."])
                .output()
        };
        
        if let Ok(output) = output {
            let content = String::from_utf8_lossy(&output.stdout);
            for line in content.lines().take(100) {
                // Parse format: file:line:content
                let parts: Vec<&str> = line.splitn(3, ':').collect();
                if parts.len() >= 3 {
                    let file = parts[0];
                    let line_num: Option<usize> = parts[1].parse().ok();
                    let content = parts[2];
                    
                    let display = format!("{}:{}: {}", file, parts[1], content);
                    
                    self.items.push(PickerItem {
                        display,
                        path: Some(PathBuf::from(file)),
                        buffer_id: None,
                        line_number: line_num,
                        match_text: Some(content.to_string()),
                    });
                }
            }
        }
        
        self.filtered_items = (0..self.items.len()).collect();
        Ok(())
    }
} 