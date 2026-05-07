use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snippet {
    pub id: Uuid,
    pub title: String,
    pub language: String,
    pub tags: Vec<String>,
    pub code: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Snippet {
    pub fn new(title: String, language: String, tags: Vec<String>, code: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            language,
            tags,
            code,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Return a language badge color as a ratatui Color.
pub fn language_color(lang: &str) -> ratatui::style::Color {
    use ratatui::style::Color;
    match lang.to_lowercase().as_str() {
        "rust" => Color::Rgb(255, 140, 0),   // orange
        "python" => Color::Rgb(70, 130, 230), // blue
        "bash" | "shell" | "sh" => Color::Rgb(80, 200, 80), // green
        "javascript" | "js" => Color::Rgb(240, 220, 60), // yellow
        "typescript" | "ts" => Color::Rgb(50, 120, 220), // ts blue
        "go" | "golang" => Color::Rgb(0, 173, 216),
        "c" | "c++" | "cpp" => Color::Rgb(100, 100, 220),
        "git" => Color::Rgb(240, 80, 50),
        "sql" => Color::Rgb(200, 150, 50),
        "toml" | "yaml" | "json" | "config" => Color::Rgb(160, 160, 160),
        _ => Color::Rgb(0, 255, 136), // phosphor green default
    }
}
