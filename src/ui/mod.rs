pub mod dashboard;

use anyhow::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::config::Config;
use crate::core::BufferManager;
use crate::modes::ModeManager;

pub use dashboard::Dashboard;

pub struct UI {
    config: Config,
}

impl UI {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }
    
    pub fn render(
        &self,
        frame: &mut Frame,
        buffer_manager: &BufferManager,
        mode_manager: &ModeManager,
        area: Rect,
    ) {
        // Create layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),      // Editor area
                Constraint::Length(1),   // Status line (if enabled)
            ])
            .split(area);
        
        // Render editor
        self.render_editor(frame, buffer_manager, chunks[0]);
        
        // Render status line if enabled
        if self.config.ui.show_status_line {
            self.render_status_line(frame, buffer_manager, mode_manager, chunks[1]);
        }
    }
    
    fn render_editor(&self, frame: &mut Frame, buffer_manager: &BufferManager, area: Rect) {
        if let Some(buffer) = buffer_manager.current_buffer() {
            let cursor_pos = buffer.cursor.position();
            
            // Calculate viewport
            let visible_lines = area.height as usize;
            let start_line = if cursor_pos.row >= visible_lines {
                cursor_pos.row - visible_lines + 1
            } else {
                0
            };
            let end_line = (start_line + visible_lines).min(buffer.content.len());
            
            // Prepare content
            let mut lines = Vec::new();
            for (i, line) in buffer.content[start_line..end_line].iter().enumerate() {
                let line_number = start_line + i;
                let is_cursor_line = line_number == cursor_pos.row;
                
                // Add line numbers if enabled
                let content = if self.config.ui.show_line_numbers {
                    format!("{:4} {}", line_number + 1, line)
                } else {
                    line.clone()
                };
                
                let style = if is_cursor_line {
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default()
                };
                
                lines.push(Line::from(Span::styled(content, style)));
            }
            
            let paragraph = Paragraph::new(lines)
                .block(Block::default().borders(Borders::NONE))
                .wrap(ratatui::widgets::Wrap { trim: !self.config.ui.wrap_lines });
            
            frame.render_widget(paragraph, area);
            
            // Render cursor
            if let Some(line) = buffer.content.get(cursor_pos.row) {
                let line_offset = if cursor_pos.row >= start_line && cursor_pos.row < end_line {
                    cursor_pos.row - start_line
                } else {
                    return; // Cursor not visible
                };
                
                let col_offset = if self.config.ui.show_line_numbers {
                    cursor_pos.col + 5 // Account for line numbers
                } else {
                    cursor_pos.col
                };
                
                let cursor_x = area.x + col_offset as u16;
                let cursor_y = area.y + line_offset as u16;
                
                if cursor_x < area.x + area.width && cursor_y < area.y + area.height {
                    frame.set_cursor(cursor_x, cursor_y);
                }
            }
        } else {
            // No buffer open
            let placeholder = Paragraph::new("No file open")
                .style(Style::default().fg(Color::DarkGray))
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(placeholder, area);
        }
    }
    
    fn render_status_line(
        &self,
        frame: &mut Frame,
        buffer_manager: &BufferManager,
        mode_manager: &ModeManager,
        area: Rect,
    ) {
        let mut spans = vec![
            Span::styled(
                format!(" {} ", mode_manager.current_mode()),
                Style::default().bg(Color::Blue).fg(Color::White),
            ),
            Span::raw(" "),
        ];
        
        if let Some(buffer) = buffer_manager.current_buffer() {
            // File name
            spans.push(Span::styled(
                &buffer.name,
                Style::default().fg(Color::White),
            ));
            
            // Modified indicator
            if buffer.modified {
                spans.push(Span::styled(" [+]", Style::default().fg(Color::Yellow)));
            }
            
            // Cursor position
            let pos = buffer.cursor.position();
            spans.push(Span::raw(format!(" {}:{} ", pos.row + 1, pos.col + 1)));
        }
        
        let status_line = Paragraph::new(Line::from(spans))
            .style(Style::default().bg(Color::DarkGray));
        
        frame.render_widget(status_line, area);
    }
} 