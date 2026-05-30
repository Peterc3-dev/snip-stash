mod snippet;
mod storage;

use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table};
use ratatui::Terminal;
use snippet::Snippet;

const GREEN: Color = Color::Rgb(0, 255, 200);
const DIM_GREEN: Color = Color::Rgb(0, 128, 100);

#[derive(PartialEq)]
enum Mode {
    Browse,
    View,
    Search,
}

struct App {
    snippets: Vec<Snippet>,
    cursor: usize,
    code_scroll: usize,
    mode: Mode,
    search: String,
}

impl App {
    fn new() -> Self {
        let mut snippets = storage::load_snippets();
        if snippets.is_empty() {
            snippets = storage::starter_snippets();
            let _ = storage::save_snippets(&snippets);
        }
        Self {
            snippets,
            cursor: 0,
            code_scroll: 0,
            mode: Mode::Browse,
            search: String::new(),
        }
    }

    fn filtered(&self) -> Vec<(usize, &Snippet)> {
        filter_snippets(&self.snippets, &self.search)
    }

    fn selected(&self) -> Option<&Snippet> {
        let f = self.filtered();
        f.get(self.cursor).map(|(_, s)| *s)
    }
}

/// Return true if `snippet` matches the lowercased query `q`.
///
/// Matches against title, language, any tag, or the code body. An empty
/// query matches everything (handled by the caller for efficiency).
fn snippet_matches(snippet: &Snippet, q: &str) -> bool {
    snippet.title.to_lowercase().contains(q)
        || snippet.language.to_lowercase().contains(q)
        || snippet.tags.iter().any(|t| t.to_lowercase().contains(q))
        || snippet.code.to_lowercase().contains(q)
}

/// Filter `snippets` by `search`, preserving each snippet's original index.
///
/// An empty/whitespace-only search returns every snippet. The search is
/// case-insensitive.
fn filter_snippets<'a>(snippets: &'a [Snippet], search: &str) -> Vec<(usize, &'a Snippet)> {
    if search.is_empty() {
        snippets.iter().enumerate().collect()
    } else {
        let q = search.to_lowercase();
        snippets
            .iter()
            .enumerate()
            .filter(|(_, s)| snippet_matches(s, &q))
            .collect()
    }
}

fn main() -> io::Result<()> {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        original_hook(info);
    }));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let result = run(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
                .split(f.area());

            draw_list(f, chunks[0], app);
            draw_preview(f, chunks[1], app);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match app.mode {
                    Mode::Search => match key.code {
                        KeyCode::Esc => {
                            app.mode = Mode::Browse;
                            app.search.clear();
                            app.cursor = 0;
                        }
                        KeyCode::Enter => app.mode = Mode::Browse,
                        KeyCode::Char(c) => {
                            app.search.push(c);
                            app.cursor = 0;
                        }
                        KeyCode::Backspace => {
                            app.search.pop();
                            app.cursor = 0;
                        }
                        _ => {}
                    },
                    Mode::View => match key.code {
                        KeyCode::Esc | KeyCode::Char('q') => app.mode = Mode::Browse,
                        KeyCode::Char('j') | KeyCode::Down => app.code_scroll += 1,
                        KeyCode::Char('k') | KeyCode::Up => {
                            app.code_scroll = app.code_scroll.saturating_sub(1);
                        }
                        _ => {}
                    },
                    Mode::Browse => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(());
                        }
                        KeyCode::Char('j') | KeyCode::Down => {
                            let max = app.filtered().len().saturating_sub(1);
                            if app.cursor < max {
                                app.cursor += 1;
                                app.code_scroll = 0;
                            }
                        }
                        KeyCode::Char('k') | KeyCode::Up if app.cursor > 0 => {
                            app.cursor -= 1;
                            app.code_scroll = 0;
                        }
                        KeyCode::Enter => {
                            app.mode = Mode::View;
                            app.code_scroll = 0;
                        }
                        KeyCode::Char('/') => {
                            app.mode = Mode::Search;
                            app.search.clear();
                        }
                        KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            let filtered = app.filtered();
                            if let Some(&(real_idx, _)) = filtered.get(app.cursor) {
                                app.snippets.remove(real_idx);
                                let _ = storage::save_snippets(&app.snippets);
                                if app.cursor > 0 && app.cursor >= app.filtered().len() {
                                    app.cursor -= 1;
                                }
                            }
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}

fn draw_list(f: &mut ratatui::Frame, area: Rect, app: &App) {
    let filtered = app.filtered();
    let header = Row::new(vec![
        Cell::from("Title").style(Style::default().fg(GREEN).add_modifier(Modifier::BOLD)),
        Cell::from("Lang").style(Style::default().fg(GREEN).add_modifier(Modifier::BOLD)),
    ]);

    let rows: Vec<Row> = filtered
        .iter()
        .enumerate()
        .map(|(i, (_, s))| {
            let style = if i == app.cursor {
                Style::default().fg(Color::Black).bg(GREEN)
            } else {
                Style::default().fg(GREEN)
            };
            Row::new(vec![
                Cell::from(s.title.clone()).style(style),
                Cell::from(s.language.clone())
                    .style(Style::default().fg(snippet::language_color(&s.language))),
            ])
        })
        .collect();

    let title = if app.mode == Mode::Search {
        format!("Snippets /{}", app.search)
    } else {
        format!("Snippets ({})", filtered.len())
    };

    let table = Table::new(rows, [Constraint::Min(20), Constraint::Length(12)])
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(if app.mode == Mode::Search {
                    GREEN
                } else {
                    DIM_GREEN
                }))
                .title(Span::styled(title, Style::default().fg(GREEN))),
        );
    f.render_widget(table, area);
}

fn draw_preview(f: &mut ratatui::Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if app.mode == Mode::View {
            GREEN
        } else {
            DIM_GREEN
        }))
        .title(Span::styled("Preview", Style::default().fg(GREEN)));

    if let Some(s) = app.selected() {
        let inner = block.inner(area);
        f.render_widget(block, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(3),
                Constraint::Length(1),
            ])
            .split(inner);

        // Metadata
        let tags_str = s.tags.join(", ");
        let meta = Paragraph::new(vec![
            Line::from(vec![
                Span::styled(
                    &s.title,
                    Style::default().fg(GREEN).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("  [{}]", s.language),
                    Style::default().fg(snippet::language_color(&s.language)),
                ),
            ]),
            Line::from(Span::styled(
                format!("tags: {}", tags_str),
                Style::default().fg(DIM_GREEN),
            )),
        ]);
        f.render_widget(meta, chunks[0]);

        // Code
        let code_lines: Vec<Line> = s
            .code
            .lines()
            .enumerate()
            .skip(app.code_scroll)
            .map(|(i, l)| {
                Line::from(vec![
                    Span::styled(format!("{:3} ", i + 1), Style::default().fg(DIM_GREEN)),
                    Span::styled(l, Style::default().fg(GREEN)),
                ])
            })
            .collect();
        let code = Paragraph::new(code_lines);
        f.render_widget(code, chunks[1]);

        // Footer hint
        let hint = if app.mode == Mode::View {
            "Esc: back  j/k: scroll"
        } else {
            "Enter: view  /: search  Ctrl-D: delete"
        };
        f.render_widget(
            Paragraph::new(Span::styled(hint, Style::default().fg(DIM_GREEN))),
            chunks[2],
        );
    } else {
        let empty = Paragraph::new(Span::styled(
            "No snippet selected",
            Style::default().fg(DIM_GREEN),
        ))
        .block(block);
        f.render_widget(empty, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<Snippet> {
        vec![
            Snippet::new(
                "Rust Error Handling".into(),
                "rust".into(),
                vec!["error-handling".into()],
                "fn read_config() -> Result<(), ()> { Ok(()) }".into(),
            ),
            Snippet::new(
                "Grep Patterns".into(),
                "bash".into(),
                vec!["grep".into(), "search".into()],
                "grep -rn 'TODO' .".into(),
            ),
            Snippet::new(
                "Python Comprehension".into(),
                "python".into(),
                vec!["functional".into()],
                "[x for x in range(10)]".into(),
            ),
        ]
    }

    #[test]
    fn empty_search_returns_all_with_original_indices() {
        let snips = sample();
        let result = filter_snippets(&snips, "");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].0, 0);
        assert_eq!(result[2].0, 2);
    }

    #[test]
    fn filter_by_language_is_case_insensitive() {
        let snips = sample();
        let result = filter_snippets(&snips, "RUST");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1.title, "Rust Error Handling");
    }

    #[test]
    fn filter_preserves_original_index_after_skipping() {
        let snips = sample();
        // "python" only matches the third snippet (original index 2).
        let result = filter_snippets(&snips, "python");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, 2);
    }

    #[test]
    fn filter_matches_tags_and_code_body() {
        let snips = sample();
        assert_eq!(filter_snippets(&snips, "error-handling").len(), 1);
        assert_eq!(filter_snippets(&snips, "todo").len(), 1); // code body match
    }

    #[test]
    fn filter_no_match_returns_empty() {
        let snips = sample();
        assert!(filter_snippets(&snips, "zzznomatch").is_empty());
    }

    #[test]
    fn snippet_matches_title_substring() {
        let snips = sample();
        assert!(snippet_matches(&snips[1], "grep"));
        assert!(!snippet_matches(&snips[1], "rust"));
    }
}
