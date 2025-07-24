use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::config::Config;

const ZEN_VIM_ART: &[&str] = &[
    "                                                                     ",
    "       ████████╗███████╗███╗   ██╗      ██╗   ██╗██╗███╗   ███╗     ",
    "       ╚══██╔══╝██╔════╝████╗  ██║      ██║   ██║██║████╗ ████║     ",
    "          ██║   █████╗  ██╔██╗ ██║█████╗██║   ██║██║██╔████╔██║     ",
    "          ██║   ██╔══╝  ██║╚██╗██║╚════╝╚██╗ ██╔╝██║██║╚██╔╝██║     ",
    "          ██║   ███████╗██║ ╚████║       ╚████╔╝ ██║██║ ╚═╝ ██║     ",
    "          ╚═╝   ╚══════╝╚═╝  ╚═══╝        ╚═══╝  ╚═╝╚═╝     ╚═╝     ",
    "                                                                     ",
    "                    🧘 Minimalist • Fast • Zen                      ",
    "                                                                     ",
];

const MENU_ITEMS: &[(&str, &str, &str)] = &[
    ("pf", "Find Files", "files"),
    ("pt", "Grep Text", "grep"),
    ("pb", "Buffers", "buffers"),
    ("pr", "Resume Session", "resume"),
    ("q", "Quit", "quit"),
];

pub struct Dashboard {
    config: Config,
}

impl Dashboard {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }
    
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        // Create vertical layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),           // Top padding
                Constraint::Length(ZEN_VIM_ART.len() as u16), // ASCII art
                Constraint::Length(2),           // Separator
                Constraint::Length(MENU_ITEMS.len() as u16), // Menu items
                Constraint::Length(3),           // Instructions
                Constraint::Min(1),              // Bottom padding
            ])
            .split(area);
        
        // Render ASCII art
        let art_lines: Vec<Line> = ZEN_VIM_ART
            .iter()
            .map(|line| {
                Line::from(Span::styled(
                    *line,
                    Style::default().fg(Color::Cyan),
                ))
            })
            .collect();
        
        let art_paragraph = Paragraph::new(art_lines)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::NONE));
        
        frame.render_widget(art_paragraph, chunks[1]);
        
        // Render menu items
        let menu_lines: Vec<Line> = MENU_ITEMS
            .iter()
            .map(|(key, desc, _)| {
                Line::from(vec![
                    Span::raw("    ["),
                    Span::styled(*key, Style::default().fg(Color::Yellow)),
                    Span::raw("]  "),
                    Span::styled(*desc, Style::default().fg(Color::White)),
                ])
            })
            .collect();
        
        let menu_paragraph = Paragraph::new(menu_lines)
            .block(Block::default().borders(Borders::NONE));
        
        frame.render_widget(menu_paragraph, chunks[3]);
        
        // Render instructions
        let instructions = vec![
            Line::from(vec![
                Span::raw("    Leader key: "),
                Span::styled("<space>", Style::default().fg(Color::Green)),
            ]),
            Line::from(Span::styled(
                "    Press any menu key to continue...",
                Style::default().fg(Color::DarkGray),
            )),
        ];
        
        let instructions_paragraph = Paragraph::new(instructions)
            .block(Block::default().borders(Borders::NONE));
        
        frame.render_widget(instructions_paragraph, chunks[4]);
    }
    
    pub fn handle_key(&mut self, key: KeyEvent) -> Option<String> {
        match key.code {
            KeyCode::Char(c) => {
                // Check if it matches any menu item
                for (menu_key, _, action) in MENU_ITEMS {
                    if menu_key.chars().any(|ch| ch == c) {
                        return Some(action.to_string());
                    }
                }
                None
            }
            _ => None,
        }
    }
} 