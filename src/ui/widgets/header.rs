use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Text},
    widgets::{Paragraph, Widget},
};

use crate::ui::theme;

const LOGO: &str = r#"
 ██   ██  ██████  ██  ██  ██████  ████    ██████  ██████  ██████
 ███ ███  ██  ██  ██  ██  ██  ██  ██ ██   ██  ██  ██      ██  ██
 ██ █ ██  ██████  ██████  ██  ██  ████    ██████  ██ ███  ██████
 ██   ██  ██  ██  ██  ██  ██████  ██ ██   ██  ██  ██████  ██  ██
"#;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Header {
    show_logo: bool,
}

impl Header {
    pub fn new(show_logo: bool) -> Self {
        Self { show_logo }
    }
}

impl Widget for Header {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.show_logo {
            let logo_lines: Vec<Line> = LOGO
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| Line::styled(line, Style::default().fg(theme::PRIMARY)))
                .collect();

            let version_line = Line::styled(
                format!(" v{}", VERSION),
                Style::default().fg(theme::SECONDARY),
            );

            let mut lines = logo_lines;
            lines.push(version_line);

            let text = Text::from(lines);
            let paragraph = Paragraph::new(text);
            paragraph.render(area, buf);
        } else {
            // Compact header for smaller terminals
            let line = Line::styled(
                format!("MAHORAGA v{}", VERSION),
                Style::default().fg(theme::PRIMARY),
            );
            let paragraph = Paragraph::new(line);
            paragraph.render(area, buf);
        }
    }
}

impl Header {
    /// Get the height of the header
    pub fn height(show_logo: bool) -> u16 {
        if show_logo {
            5 // Logo (4 lines) + version
        } else {
            1
        }
    }
}
