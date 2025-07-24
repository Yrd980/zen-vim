#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone)]
pub struct Cursor {
    position: Position,
    desired_col: usize, // For vertical movement
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            position: Position { row: 0, col: 0 },
            desired_col: 0,
        }
    }
    
    pub fn position(&self) -> Position {
        self.position
    }
    
    pub fn move_to_position(&mut self, pos: Position) {
        self.position = pos;
        self.desired_col = pos.col;
    }
    
    pub fn move_to_column(&mut self, col: usize) {
        self.position.col = col;
        self.desired_col = col;
    }
    
    pub fn move_left(&mut self, content: &[String]) {
        if self.position.col > 0 {
            self.position.col -= 1;
            self.desired_col = self.position.col;
        } else if self.position.row > 0 {
            // Move to end of previous line
            self.position.row -= 1;
            self.position.col = content.get(self.position.row)
                .map(|s| s.chars().count())
                .unwrap_or(0);
            self.desired_col = self.position.col;
        }
    }
    
    pub fn move_right(&mut self, content: &[String]) {
        if let Some(line) = content.get(self.position.row) {
            let line_len = line.chars().count();
            if self.position.col < line_len {
                self.position.col += 1;
                self.desired_col = self.position.col;
            } else if self.position.row + 1 < content.len() {
                // Move to start of next line
                self.position.row += 1;
                self.position.col = 0;
                self.desired_col = 0;
            }
        }
    }
    
    pub fn move_up(&mut self, content: &[String]) {
        if self.position.row > 0 {
            self.position.row -= 1;
            self.position.col = self.clamp_column(content, self.desired_col);
        }
    }
    
    pub fn move_down(&mut self, content: &[String]) {
        if self.position.row + 1 < content.len() {
            self.position.row += 1;
            self.position.col = self.clamp_column(content, self.desired_col);
        }
    }
    
    pub fn move_word_forward(&mut self, content: &[String]) {
        if let Some(line) = content.get(self.position.row) {
            let chars: Vec<char> = line.chars().collect();
            let mut pos = self.position.col;
            
            // Skip current word
            while pos < chars.len() && chars[pos].is_alphanumeric() {
                pos += 1;
            }
            
            // Skip whitespace
            while pos < chars.len() && chars[pos].is_whitespace() {
                pos += 1;
            }
            
            if pos < chars.len() {
                self.position.col = pos;
                self.desired_col = pos;
            } else if self.position.row + 1 < content.len() {
                // Move to next line
                self.position.row += 1;
                self.position.col = 0;
                self.desired_col = 0;
            }
        }
    }
    
    pub fn move_to_end_of_word(&mut self, content: &[String]) {
        if let Some(line) = content.get(self.position.row) {
            let chars: Vec<char> = line.chars().collect();
            if chars.is_empty() {
                return;
            }
            
            let mut pos = self.position.col;
            
            // If we're at the end of a word, move to the end of the next word
            if pos < chars.len() && chars[pos].is_alphanumeric() {
                // Skip current word
                while pos < chars.len() && chars[pos].is_alphanumeric() {
                    pos += 1;
                }
            }
            
            // Skip whitespace to get to next word
            while pos < chars.len() && !chars[pos].is_alphanumeric() {
                pos += 1;
            }
            
            // Move to end of the word
            while pos < chars.len() && chars[pos].is_alphanumeric() {
                pos += 1;
            }
            
            // Back up one to be ON the last character of the word
            if pos > 0 {
                pos -= 1;
            }
            
            // Make sure we don't go past the line
            pos = pos.min(chars.len().saturating_sub(1));
            
            self.position.col = pos;
            self.desired_col = pos;
        }
    }
    
    pub fn move_word_backward(&mut self, content: &[String]) {
        if let Some(line) = content.get(self.position.row) {
            let chars: Vec<char> = line.chars().collect();
            let mut pos = if self.position.col > 0 {
                self.position.col - 1
            } else {
                return;
            };
            
            // Skip whitespace
            while pos > 0 && chars[pos].is_whitespace() {
                pos -= 1;
            }
            
            // Skip current word
            while pos > 0 && chars[pos].is_alphanumeric() {
                pos -= 1;
            }
            
            // Adjust if we went too far
            if pos < chars.len() && !chars[pos].is_alphanumeric() && pos + 1 < chars.len() {
                pos += 1;
            }
            
            self.position.col = pos;
            self.desired_col = pos;
        }
    }
    
    fn clamp_column(&self, content: &[String], desired_col: usize) -> usize {
        if let Some(line) = content.get(self.position.row) {
            let line_len = line.chars().count();
            desired_col.min(line_len)
        } else {
            0
        }
    }
} 