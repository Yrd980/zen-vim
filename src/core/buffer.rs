use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::cursor::{Cursor, Position};

#[derive(Debug, Clone)]
pub struct Buffer {
    pub id: usize,
    pub path: Option<PathBuf>,
    pub content: Vec<String>,
    pub cursor: Cursor,
    pub modified: bool,
    pub name: String,
    undo_stack: Vec<Vec<String>>,
    redo_stack: Vec<Vec<String>>,
}

impl Buffer {
    pub fn new(id: usize, name: String) -> Self {
        Self {
            id,
            path: None,
            content: vec![String::new()],
            cursor: Cursor::new(),
            modified: false,
            name,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }
    
    pub fn from_file<P: AsRef<Path>>(id: usize, path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let content = if path.exists() {
            std::fs::read_to_string(&path)?
                .lines()
                .map(|s| s.to_string())
                .collect()
        } else {
            vec![String::new()]
        };
        
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("untitled")
            .to_string();
            
        Ok(Self {
            id,
            path: Some(path),
            content,
            cursor: Cursor::new(),
            modified: false,
            name,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        })
    }
    
    pub fn save(&mut self) -> Result<()> {
        if let Some(ref path) = self.path {
            let content = self.content.join("\n");
            std::fs::write(path, content)?;
            self.modified = false;
            Ok(())
        } else {
            Err(anyhow!("No file path set"))
        }
    }
    
    pub fn save_as<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref().to_path_buf();
        let content = self.content.join("\n");
        std::fs::write(&path, content)?;
        self.path = Some(path);
        self.modified = false;
        Ok(())
    }
    
    fn push_undo(&mut self) {
        if self.undo_stack.len() > 100 {
            self.undo_stack.remove(0);
        }
        self.undo_stack.push(self.content.clone());
        self.redo_stack.clear();
    }
    
    pub fn insert_char(&mut self, ch: char) {
        self.push_undo();
        let pos = self.cursor.position();
        if pos.row < self.content.len() {
            let line = &mut self.content[pos.row];
            let byte_pos = line.char_indices()
                .nth(pos.col)
                .map(|(i, _)| i)
                .unwrap_or(line.len());
            line.insert(byte_pos, ch);
            self.cursor.move_right(&self.content);
            self.modified = true;
        }
    }
    
    pub fn insert_newline(&mut self) {
        self.push_undo();
        let pos = self.cursor.position();
        if pos.row < self.content.len() {
            let line = &self.content[pos.row];
            let byte_pos = line.char_indices()
                .nth(pos.col)
                .map(|(i, _)| i)
                .unwrap_or(line.len());
            
            let new_line = line[byte_pos..].to_string();
            self.content[pos.row] = line[..byte_pos].to_string();
            self.content.insert(pos.row + 1, new_line);
            
            self.cursor.move_down(&self.content);
            self.cursor.move_to_column(0);
            self.modified = true;
        }
    }
    
    pub fn backspace(&mut self) {
        self.push_undo();
        let pos = self.cursor.position();
        
        if pos.col > 0 {
            // Delete character in current line
            let line = &mut self.content[pos.row];
            let char_indices: Vec<_> = line.char_indices().collect();
            if pos.col - 1 < char_indices.len() {
                let byte_pos = char_indices[pos.col - 1].0;
                let next_byte_pos = char_indices.get(pos.col).map(|(i, _)| *i).unwrap_or(line.len());
                line.drain(byte_pos..next_byte_pos);
                self.cursor.move_left(&self.content);
                self.modified = true;
            }
        } else if pos.row > 0 {
            // Merge with previous line
            let current_line = self.content.remove(pos.row);
            let prev_line_len = self.content[pos.row - 1].chars().count();
            self.content[pos.row - 1].push_str(&current_line);
            self.cursor.move_up(&self.content);
            self.cursor.move_to_column(prev_line_len);
            self.modified = true;
        }
    }
    
    pub fn delete_char(&mut self) {
        self.push_undo();
        let pos = self.cursor.position();
        
        if pos.row < self.content.len() {
            let line = &mut self.content[pos.row];
            let char_indices: Vec<_> = line.char_indices().collect();
            
            if pos.col < char_indices.len() {
                // Delete character at cursor
                let byte_pos = char_indices[pos.col].0;
                let next_byte_pos = char_indices.get(pos.col + 1).map(|(i, _)| *i).unwrap_or(line.len());
                line.drain(byte_pos..next_byte_pos);
                self.modified = true;
            } else if pos.row + 1 < self.content.len() {
                // Merge with next line
                let next_line = self.content.remove(pos.row + 1);
                self.content[pos.row].push_str(&next_line);
                self.modified = true;
            }
        }
    }
    
    pub fn delete_line(&mut self) {
        self.push_undo();
        let pos = self.cursor.position();
        
        if !self.content.is_empty() {
            if self.content.len() == 1 {
                self.content[0].clear();
            } else {
                self.content.remove(pos.row);
                if pos.row >= self.content.len() && pos.row > 0 {
                    self.cursor.move_up(&self.content);
                }
            }
            self.cursor.move_to_column(0);
            self.modified = true;
        }
    }
    
    pub fn undo(&mut self) {
        if let Some(previous_content) = self.undo_stack.pop() {
            self.redo_stack.push(self.content.clone());
            self.content = previous_content;
            self.modified = true;
        }
    }
    
    pub fn redo(&mut self) {
        if let Some(next_content) = self.redo_stack.pop() {
            self.undo_stack.push(self.content.clone());
            self.content = next_content;
            self.modified = true;
        }
    }
    
    pub fn line_count(&self) -> usize {
        self.content.len()
    }
    
    pub fn get_line(&self, row: usize) -> Option<&String> {
        self.content.get(row)
    }
}

pub struct BufferManager {
    buffers: HashMap<usize, Buffer>,
    current_buffer_id: Option<usize>,
    next_id: usize,
}

impl BufferManager {
    pub fn new() -> Self {
        Self {
            buffers: HashMap::new(),
            current_buffer_id: None,
            next_id: 1,
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.buffers.is_empty()
    }
    
    pub fn create_buffer(&mut self, name: String) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        
        let buffer = Buffer::new(id, name);
        self.buffers.insert(id, buffer);
        self.current_buffer_id = Some(id);
        
        id
    }
    
    pub fn open_file<P: AsRef<Path>>(&mut self, path: P) -> Result<usize> {
        let id = self.next_id;
        self.next_id += 1;
        
        let buffer = Buffer::from_file(id, path)?;
        self.buffers.insert(id, buffer);
        self.current_buffer_id = Some(id);
        
        Ok(id)
    }
    
    pub fn current_buffer(&self) -> Option<&Buffer> {
        self.current_buffer_id.and_then(|id| self.buffers.get(&id))
    }
    
    pub fn current_buffer_mut(&mut self) -> Option<&mut Buffer> {
        self.current_buffer_id.and_then(|id| self.buffers.get_mut(&id))
    }
    
    pub fn switch_buffer(&mut self, id: usize) -> bool {
        if self.buffers.contains_key(&id) {
            self.current_buffer_id = Some(id);
            true
        } else {
            false
        }
    }
    
    pub fn close_buffer(&mut self, id: usize) -> Result<()> {
        if let Some(buffer) = self.buffers.get(&id) {
            if buffer.modified {
                return Err(anyhow!("Buffer has unsaved changes"));
            }
        }
        
        self.buffers.remove(&id);
        
        if self.current_buffer_id == Some(id) {
            self.current_buffer_id = self.buffers.keys().next().copied();
        }
        
        Ok(())
    }
    
    pub fn list_buffers(&self) -> Vec<&Buffer> {
        self.buffers.values().collect()
    }
    
    // Delegated cursor operations for current buffer
    pub fn move_cursor_left(&mut self) {
        if let Some(buffer) = self.current_buffer_mut() {
            buffer.cursor.move_left(&buffer.content);
        }
    }
    
    pub fn move_cursor_right(&mut self) {
        if let Some(buffer) = self.current_buffer_mut() {
            buffer.cursor.move_right(&buffer.content);
        }
    }
    
    pub fn move_cursor_up(&mut self) {
        if let Some(buffer) = self.current_buffer_mut() {
            buffer.cursor.move_up(&buffer.content);
        }
    }
    
    pub fn move_cursor_down(&mut self) {
        if let Some(buffer) = self.current_buffer_mut() {
            buffer.cursor.move_down(&buffer.content);
        }
    }
    
    pub fn move_word_forward(&mut self) {
        if let Some(buffer) = self.current_buffer_mut() {
            buffer.cursor.move_word_forward(&buffer.content);
        }
    }
    
    pub fn move_word_backward(&mut self) {
        if let Some(buffer) = self.current_buffer_mut() {
            buffer.cursor.move_word_backward(&buffer.content);
        }
    }
    
    pub fn move_to_line_start(&mut self) {
        if let Some(buffer) = self.current_buffer_mut() {
            buffer.cursor.move_to_column(0);
        }
    }
    
    pub fn move_to_line_end(&mut self) {
        if let Some(buffer) = self.current_buffer_mut() {
            let line_len = buffer.content.get(buffer.cursor.position().row)
                .map(|s| s.chars().count())
                .unwrap_or(0);
            buffer.cursor.move_to_column(line_len);
        }
    }
    
    pub fn move_to_file_start(&mut self) {
        if let Some(buffer) = self.current_buffer_mut() {
            buffer.cursor.move_to_position(Position { row: 0, col: 0 });
        }
    }
    
    pub fn move_to_file_end(&mut self) {
        if let Some(buffer) = self.current_buffer_mut() {
            let last_row = buffer.content.len().saturating_sub(1);
            let last_col = buffer.content.get(last_row)
                .map(|s| s.chars().count())
                .unwrap_or(0);
            buffer.cursor.move_to_position(Position { row: last_row, col: last_col });
        }
    }
    
    pub fn insert_char(&mut self, ch: char) {
        if let Some(buffer) = self.current_buffer_mut() {
            buffer.insert_char(ch);
        }
    }
    
    pub fn insert_newline(&mut self) {
        if let Some(buffer) = self.current_buffer_mut() {
            buffer.insert_newline();
        }
    }
    
    pub fn insert_tab(&mut self) {
        if let Some(buffer) = self.current_buffer_mut() {
            buffer.insert_char('\t');
        }
    }
    
    pub fn backspace(&mut self) {
        if let Some(buffer) = self.current_buffer_mut() {
            buffer.backspace();
        }
    }
    
    pub fn delete_char(&mut self) {
        if let Some(buffer) = self.current_buffer_mut() {
            buffer.delete_char();
        }
    }
    
    pub fn delete_line(&mut self) {
        if let Some(buffer) = self.current_buffer_mut() {
            buffer.delete_line();
        }
    }
    
    pub fn insert_line_below(&mut self) {
        if let Some(buffer) = self.current_buffer_mut() {
            let pos = buffer.cursor.position();
            buffer.content.insert(pos.row + 1, String::new());
            buffer.cursor.move_down(&buffer.content);
            buffer.cursor.move_to_column(0);
            buffer.modified = true;
        }
    }
    
    pub fn insert_line_above(&mut self) {
        if let Some(buffer) = self.current_buffer_mut() {
            let pos = buffer.cursor.position();
            buffer.content.insert(pos.row, String::new());
            buffer.cursor.move_to_column(0);
            buffer.modified = true;
        }
    }
    
    pub fn undo(&mut self) {
        if let Some(buffer) = self.current_buffer_mut() {
            buffer.undo();
        }
    }
    
    pub fn redo(&mut self) {
        if let Some(buffer) = self.current_buffer_mut() {
            buffer.redo();
        }
    }
    
    pub fn save_current(&mut self) -> Result<()> {
        if let Some(buffer) = self.current_buffer_mut() {
            buffer.save()
        } else {
            Err(anyhow!("No current buffer"))
        }
    }
    
    pub fn rename_current_file(&mut self) -> Result<()> {
        // TODO: Implement file renaming with UI input
        Ok(())
    }
    
    pub fn resume_session(&mut self) -> Result<()> {
        // TODO: Implement session restoration
        Ok(())
    }
} 