use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::core::BufferManager;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
    Command,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Normal => write!(f, "NORMAL"),
            Mode::Insert => write!(f, "INSERT"),
            Mode::Visual => write!(f, "VISUAL"),
            Mode::Command => write!(f, "COMMAND"),
        }
    }
}

pub struct ModeManager {
    current_mode: Mode,
    last_mode: Mode,
    command_buffer: String,
    last_search_pattern: String,
}

impl ModeManager {
    pub fn new() -> Self {
        Self {
            current_mode: Mode::Normal,
            last_mode: Mode::Normal,
            command_buffer: String::new(),
            last_search_pattern: String::new(),
        }
    }
    
    pub fn current_mode(&self) -> Mode {
        self.current_mode
    }
    
    pub fn set_mode(&mut self, mode: Mode) {
        self.last_mode = self.current_mode;
        self.current_mode = mode;
        
        // Clear command buffer when entering command mode (unless it's a search)
        if mode == Mode::Command && !self.command_buffer.starts_with('/') {
            self.command_buffer.clear();
        }
    }
    
    pub fn set_command_mode_with_prefix(&mut self, prefix: char) {
        self.last_mode = self.current_mode;
        self.current_mode = Mode::Command;
        self.command_buffer.clear();
        self.command_buffer.push(prefix);
    }
    
    pub fn command_buffer(&self) -> &str {
        &self.command_buffer
    }
    
    pub fn handle_key(&mut self, key: KeyEvent, buffer_manager: &mut BufferManager) -> Result<()> {
        match self.current_mode {
            Mode::Normal => self.handle_normal_mode(key, buffer_manager)?,
            Mode::Insert => self.handle_insert_mode(key, buffer_manager)?,
            Mode::Visual => self.handle_visual_mode(key, buffer_manager)?,
            Mode::Command => self.handle_command_mode(key, buffer_manager)?,
        }
        Ok(())
    }
    
    fn handle_normal_mode(&mut self, key: KeyEvent, buffer_manager: &mut BufferManager) -> Result<()> {
        match key.code {
            // Movement
            KeyCode::Char('h') | KeyCode::Left => {
                buffer_manager.move_cursor_left();
            }
            KeyCode::Char('j') | KeyCode::Down => {
                buffer_manager.move_cursor_down();
            }
            KeyCode::Char('k') | KeyCode::Up => {
                buffer_manager.move_cursor_up();
            }
            KeyCode::Char('l') | KeyCode::Right => {
                buffer_manager.move_cursor_right();
            }
            
            // Word movement
            KeyCode::Char('w') => {
                buffer_manager.move_word_forward();
            }
            KeyCode::Char('W') => {
                // WORD movement (whitespace-separated)
                buffer_manager.move_word_forward(); // TODO: implement WORD vs word
            }
            KeyCode::Char('b') => {
                buffer_manager.move_word_backward();
            }
            KeyCode::Char('B') => {
                // WORD movement backward (whitespace-separated)
                buffer_manager.move_word_backward(); // TODO: implement WORD vs word
            }
            KeyCode::Char('e') => {
                // Move to end of word
                if let Some(buffer) = buffer_manager.current_buffer_mut() {
                    buffer.cursor.move_to_end_of_word(&buffer.content);
                }
            }
            KeyCode::Char('E') => {
                // Move to end of WORD (for now, same as 'e')
                if let Some(buffer) = buffer_manager.current_buffer_mut() {
                    buffer.cursor.move_to_end_of_word(&buffer.content);
                }
            }
            
            // Line navigation
            KeyCode::Char('0') => {
                buffer_manager.move_to_line_start();
            }
            KeyCode::Char('$') => {
                buffer_manager.move_to_line_end();
            }
            
            // Page navigation
            KeyCode::Char('g') => {
                // TODO: Handle gg for file start
                buffer_manager.move_to_file_start();
            }
            KeyCode::Char('G') => {
                buffer_manager.move_to_file_end();
            }
            
            // Mode switches
            KeyCode::Char('i') => {
                self.set_mode(Mode::Insert);
            }
            KeyCode::Char('I') => {
                buffer_manager.move_to_line_start();
                self.set_mode(Mode::Insert);
            }
            KeyCode::Char('a') => {
                buffer_manager.move_cursor_right();
                self.set_mode(Mode::Insert);
            }
            KeyCode::Char('A') => {
                buffer_manager.move_to_line_end();
                self.set_mode(Mode::Insert);
            }
            KeyCode::Char('o') => {
                buffer_manager.insert_line_below();
                self.set_mode(Mode::Insert);
            }
            KeyCode::Char('O') => {
                buffer_manager.insert_line_above();
                self.set_mode(Mode::Insert);
            }
            KeyCode::Char('v') => {
                self.set_mode(Mode::Visual);
            }
            KeyCode::Char(':') => {
                self.set_mode(Mode::Command);
            }
            KeyCode::Char('/') => {
                // Search mode - enter command mode with / prefix
                self.set_command_mode_with_prefix('/');
            }
            
            // Deletion
            KeyCode::Char('x') => {
                buffer_manager.delete_char();
            }
            KeyCode::Char('d') => {
                // TODO: Handle dd for line deletion
                buffer_manager.delete_line();
            }
            
            // Undo/Redo
            KeyCode::Char('u') => {
                buffer_manager.undo();
            }
            KeyCode::Char('r') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                buffer_manager.redo();
            }
            
            // Search navigation
            KeyCode::Char('n') => {
                // Next search match
                if !self.last_search_pattern.is_empty() {
                    self.search_in_buffer(&self.last_search_pattern.clone(), buffer_manager);
                }
            }
            KeyCode::Char('N') => {
                // Previous search match  
                if !self.last_search_pattern.is_empty() {
                    self.search_backward_in_buffer(&self.last_search_pattern.clone(), buffer_manager);
                }
            }
            
            // Search word under cursor
            KeyCode::Char('*') => {
                if let Some(word) = self.get_word_under_cursor(buffer_manager) {
                    self.last_search_pattern = word.clone();
                    self.search_in_buffer(&word, buffer_manager);
                }
            }
            
            // Buffer navigation
            KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                // Next buffer (Ctrl+n)
                buffer_manager.next_buffer();
            }
            KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                // Previous buffer (Ctrl+p)  
                buffer_manager.previous_buffer();
            }
            
            _ => {}
        }
        Ok(())
    }
    
    fn handle_insert_mode(&mut self, key: KeyEvent, buffer_manager: &mut BufferManager) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.set_mode(Mode::Normal);
            }
            KeyCode::Char(c) => {
                buffer_manager.insert_char(c);
            }
            KeyCode::Enter => {
                buffer_manager.insert_newline();
            }
            KeyCode::Backspace => {
                buffer_manager.backspace();
            }
            KeyCode::Delete => {
                buffer_manager.delete_char();
            }
            KeyCode::Tab => {
                buffer_manager.insert_tab();
            }
            _ => {}
        }
        Ok(())
    }
    
    fn handle_visual_mode(&mut self, key: KeyEvent, _buffer_manager: &mut BufferManager) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.set_mode(Mode::Normal);
            }
            // TODO: Implement visual mode selection
            _ => {}
        }
        Ok(())
    }
    
    fn handle_command_mode(&mut self, key: KeyEvent, buffer_manager: &mut BufferManager) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.set_mode(Mode::Normal);
            }
            KeyCode::Enter => {
                self.execute_command(&self.command_buffer.clone(), buffer_manager)?;
                self.set_mode(Mode::Normal);
            }
            KeyCode::Backspace => {
                self.command_buffer.pop();
            }
            KeyCode::Char(c) => {
                self.command_buffer.push(c);
            }
            _ => {}
        }
        Ok(())
    }
    
    fn execute_command(&mut self, command: &str, buffer_manager: &mut BufferManager) -> Result<()> {
        let trimmed = command.trim();
        
        if trimmed.starts_with('/') {
            // Search command
            let pattern = &trimmed[1..];
            if !pattern.is_empty() {
                self.last_search_pattern = pattern.to_string();
                self.search_in_buffer(pattern, buffer_manager);
            }
        } else {
            // Regular ex commands
            match trimmed {
                "q" | "quit" => {
                    // TODO: Implement proper quit - for now just return to normal
                }
                "q!" => {
                    // Force quit - TODO: implement
                }
                "w" | "write" => {
                    let _ = buffer_manager.save_current();
                }
                "wq" | "x" => {
                    let _ = buffer_manager.save_current();
                    // TODO: Should quit after save
                }
                "wq!" => {
                    let _ = buffer_manager.save_current();
                    // TODO: Force save and quit
                }
                cmd if cmd.starts_with("w ") => {
                    // Save as - extract filename
                    let filename = cmd[2..].trim();
                    if let Some(buffer) = buffer_manager.current_buffer_mut() {
                        let _ = buffer.save_as(filename);
                    }
                }
                cmd if cmd.starts_with("e ") => {
                    // Edit file - extract filename
                    let filename = cmd[2..].trim();
                    let _ = buffer_manager.open_file(filename);
                }
                _ => {
                    // Unknown command - ignore for now
                }
            }
        }
        
        Ok(())
    }
    
    fn search_in_buffer(&mut self, pattern: &str, buffer_manager: &mut BufferManager) {
        if let Some(buffer) = buffer_manager.current_buffer_mut() {
            let start_row = buffer.cursor.position().row;
            let start_col = buffer.cursor.position().col + 1; // Start search after current position
            
            // Search from current position to end of file
            for (row_idx, line) in buffer.content.iter().enumerate().skip(start_row) {
                let search_start = if row_idx == start_row { start_col } else { 0 };
                let line_from_start = &line[search_start.min(line.len())..];
                
                if let Some(pos) = line_from_start.find(pattern) {
                    // Found match - move cursor there
                    let col = search_start + pos;
                    buffer.cursor.move_to_position(crate::core::cursor::Position { 
                        row: row_idx, 
                        col 
                    });
                    return;
                }
            }
            
            // If not found from current position, search from beginning
            for (row_idx, line) in buffer.content.iter().enumerate() {
                if row_idx >= start_row {
                    break; // We already searched this area
                }
                
                if let Some(pos) = line.find(pattern) {
                    // Found match - move cursor there
                    buffer.cursor.move_to_position(crate::core::cursor::Position { 
                        row: row_idx, 
                        col: pos 
                    });
                    return;
                }
            }
            
            // Pattern not found - could show message but for now just do nothing
        }
    }
    
    fn search_backward_in_buffer(&mut self, pattern: &str, buffer_manager: &mut BufferManager) {
        if let Some(buffer) = buffer_manager.current_buffer_mut() {
            let start_row = buffer.cursor.position().row;
            let start_col = if buffer.cursor.position().col > 0 { 
                buffer.cursor.position().col - 1 
            } else { 
                0 
            };
            
            // Search backward from current position to beginning
            for row_idx in (0..=start_row).rev() {
                let line = &buffer.content[row_idx];
                let search_end = if row_idx == start_row { start_col } else { line.len() };
                let line_to_search = &line[..search_end.min(line.len())];
                
                if let Some(pos) = line_to_search.rfind(pattern) {
                    // Found match - move cursor there
                    buffer.cursor.move_to_position(crate::core::cursor::Position { 
                        row: row_idx, 
                        col: pos 
                    });
                    return;
                }
            }
            
            // If not found from current position, search from end
            for row_idx in (start_row + 1..buffer.content.len()).rev() {
                let line = &buffer.content[row_idx];
                if let Some(pos) = line.rfind(pattern) {
                    // Found match - move cursor there
                    buffer.cursor.move_to_position(crate::core::cursor::Position { 
                        row: row_idx, 
                        col: pos 
                    });
                    return;
                }
            }
            
            // Pattern not found
        }
    }
    
    fn get_word_under_cursor(&self, buffer_manager: &BufferManager) -> Option<String> {
        if let Some(buffer) = buffer_manager.current_buffer() {
            let pos = buffer.cursor.position();
            if let Some(line) = buffer.content.get(pos.row) {
                let chars: Vec<char> = line.chars().collect();
                if pos.col >= chars.len() {
                    return None;
                }
                
                // Find start of word
                let mut start = pos.col;
                while start > 0 && chars[start - 1].is_alphanumeric() {
                    start -= 1;
                }
                
                // Find end of word
                let mut end = pos.col;
                while end < chars.len() && chars[end].is_alphanumeric() {
                    end += 1;
                }
                
                if end > start {
                    let word: String = chars[start..end].iter().collect();
                    Some(word)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
} 