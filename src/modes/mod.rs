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
}

impl ModeManager {
    pub fn new() -> Self {
        Self {
            current_mode: Mode::Normal,
            last_mode: Mode::Normal,
        }
    }
    
    pub fn current_mode(&self) -> Mode {
        self.current_mode
    }
    
    pub fn set_mode(&mut self, mode: Mode) {
        self.last_mode = self.current_mode;
        self.current_mode = mode;
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
            KeyCode::Char('b') => {
                buffer_manager.move_word_backward();
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
                // TODO: Execute command
                self.set_mode(Mode::Normal);
            }
            // TODO: Implement command input
            _ => {}
        }
        Ok(())
    }
} 