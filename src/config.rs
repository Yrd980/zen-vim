use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ui: UIConfig,
    pub keymaps: KeymapConfig,
    pub picker: PickerConfig,
    pub dashboard: DashboardConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIConfig {
    pub theme: String,
    pub show_line_numbers: bool,
    pub show_status_line: bool,
    pub tab_width: usize,
    pub wrap_lines: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeymapConfig {
    pub leader: String,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PickerConfig {
    pub file_ignore_patterns: Vec<String>,
    pub max_results: usize,
    pub preview_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub show_recent_files: bool,
    pub max_recent_files: usize,
    pub custom_header: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ui: UIConfig {
                theme: "zen".to_string(),
                show_line_numbers: false,
                show_status_line: false,
                tab_width: 2,
                wrap_lines: false,
            },
            keymaps: KeymapConfig {
                leader: " ".to_string(),
                timeout_ms: 1000,
            },
            picker: PickerConfig {
                file_ignore_patterns: vec![
                    ".git".to_string(),
                    "node_modules".to_string(),
                    "target".to_string(),
                    "*.pyc".to_string(),
                ],
                max_results: 100,
                preview_enabled: true,
            },
            dashboard: DashboardConfig {
                show_recent_files: true,
                max_recent_files: 5,
                custom_header: None,
            },
        }
    }
}

impl Config {
    pub fn load(config_path: Option<PathBuf>) -> Result<Self> {
        let config_dir = match config_path {
            Some(path) => path,
            None => {
                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                PathBuf::from(home).join(".config").join("zen-vim")
            }
        };
        
        let config_file = config_dir.join("config.toml");
        
        if config_file.exists() {
            let content = std::fs::read_to_string(config_file)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            // Create default config file
            let default_config = Config::default();
            std::fs::create_dir_all(&config_dir)?;
            let toml_content = toml::to_string_pretty(&default_config)?;
            std::fs::write(config_file, toml_content)?;
            Ok(default_config)
        }
    }
    
    pub fn save(&self, config_path: Option<PathBuf>) -> Result<()> {
        let config_dir = match config_path {
            Some(path) => path,
            None => {
                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                PathBuf::from(home).join(".config").join("zen-vim")
            }
        };
        
        std::fs::create_dir_all(&config_dir)?;
        let config_file = config_dir.join("config.toml");
        let toml_content = toml::to_string_pretty(self)?;
        std::fs::write(config_file, toml_content)?;
        Ok(())
    }
} 