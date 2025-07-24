use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub buffers: Vec<BufferSession>,
    pub current_buffer_id: Option<usize>,
    pub last_directory: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferSession {
    pub id: usize,
    pub path: Option<PathBuf>,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub modified: bool,
}

pub struct SessionManager {
    session_file: PathBuf,
}

impl SessionManager {
    pub fn new() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let session_file = PathBuf::from(home)
            .join(".config")
            .join("zen-vim")
            .join("session.json");
            
        Self { session_file }
    }
    
    pub fn save(&self, session_data: &SessionData) -> Result<()> {
        if let Some(parent) = self.session_file.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let json = serde_json::to_string_pretty(session_data)?;
        std::fs::write(&self.session_file, json)?;
        
        Ok(())
    }
    
    pub fn load(&self) -> Result<Option<SessionData>> {
        if !self.session_file.exists() {
            return Ok(None);
        }
        
        let content = std::fs::read_to_string(&self.session_file)?;
        let session_data: SessionData = serde_json::from_str(&content)?;
        
        Ok(Some(session_data))
    }
    
    pub fn clear(&self) -> Result<()> {
        if self.session_file.exists() {
            std::fs::remove_file(&self.session_file)?;
        }
        Ok(())
    }
} 