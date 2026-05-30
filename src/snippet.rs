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
        "rust" => Color::Rgb(255, 140, 0),                  // orange
        "python" => Color::Rgb(70, 130, 230),               // blue
        "bash" | "shell" | "sh" => Color::Rgb(80, 200, 80), // green
        "javascript" | "js" => Color::Rgb(240, 220, 60),    // yellow
        "typescript" | "ts" => Color::Rgb(50, 120, 220),    // ts blue
        "go" | "golang" => Color::Rgb(0, 173, 216),
        "c" | "c++" | "cpp" => Color::Rgb(100, 100, 220),
        "git" => Color::Rgb(240, 80, 50),
        "sql" => Color::Rgb(200, 150, 50),
        "toml" | "yaml" | "json" | "config" => Color::Rgb(160, 160, 160),
        _ => Color::Rgb(0, 255, 136), // phosphor green default
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Color;

    #[test]
    fn known_language_is_case_insensitive() {
        assert_eq!(language_color("Rust"), Color::Rgb(255, 140, 0));
        assert_eq!(language_color("RUST"), Color::Rgb(255, 140, 0));
    }

    #[test]
    fn aliases_map_to_same_color() {
        assert_eq!(language_color("js"), language_color("javascript"));
        assert_eq!(language_color("sh"), language_color("bash"));
        assert_eq!(language_color("golang"), language_color("go"));
    }

    #[test]
    fn unknown_language_falls_back_to_phosphor_green() {
        assert_eq!(language_color("brainfuck"), Color::Rgb(0, 255, 136));
    }

    #[test]
    fn new_snippet_sets_matching_timestamps_and_unique_id() {
        let a = Snippet::new("t".into(), "rust".into(), vec![], "code".into());
        let b = Snippet::new("t".into(), "rust".into(), vec![], "code".into());
        assert_eq!(a.created_at, a.updated_at);
        assert_ne!(a.id, b.id);
    }
}
