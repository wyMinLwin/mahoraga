use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::ui::theme::{self, score_color, score_label};

pub struct ScoreDisplay {
    score: u8,
}

impl ScoreDisplay {
    pub fn new(score: u8) -> Self {
        Self { score }
    }
}

impl Widget for ScoreDisplay {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme::BORDER))
            .title(Span::styled(
                " Quality Score ",
                Style::default().fg(theme::SECONDARY),
            ));

        let inner_area = block.inner(area);
        block.render(area, buf);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Score number and label
                Constraint::Length(1), // Progress bar
            ])
            .split(inner_area);

        // Score number and label
        let color = score_color(self.score);
        let label = score_label(self.score);

        let score_line = Line::from(vec![
            Span::styled(
                format!("{}", self.score),
                Style::default().fg(color),
            ),
            Span::styled("/100 - ", Style::default().fg(theme::MUTED)),
            Span::styled(label, Style::default().fg(color)),
        ]);

        let score_paragraph = Paragraph::new(score_line);
        score_paragraph.render(layout[0], buf);

        // Progress bar
        let bar_width = layout[1].width as usize;
        let filled = (self.score as usize * bar_width) / 100;
        let empty = bar_width.saturating_sub(filled);

        let bar_line = Line::from(vec![
            Span::styled("█".repeat(filled), Style::default().fg(color)),
            Span::styled("░".repeat(empty), Style::default().fg(theme::MUTED)),
        ]);

        let bar_paragraph = Paragraph::new(bar_line);
        bar_paragraph.render(layout[1], buf);
    }
}
