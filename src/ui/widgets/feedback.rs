use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::ui::theme;

pub struct Feedback<'a> {
    improvements: &'a [String],
    unclear_parts: &'a [String],
}

impl<'a> Feedback<'a> {
    pub fn new(improvements: &'a [String], unclear_parts: &'a [String]) -> Self {
        Self {
            improvements,
            unclear_parts,
        }
    }
}

impl Widget for Feedback<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Split area for improvements and unclear parts
        let has_improvements = !self.improvements.is_empty();
        let has_unclear = !self.unclear_parts.is_empty();

        let chunks = if has_improvements && has_unclear {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area)
        } else {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)])
                .split(area)
        };

        let mut chunk_idx = 0;

        // Render improvements
        if has_improvements {
            let improvements_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme::BORDER))
                .title(Span::styled(
                    " Improvements ",
                    Style::default().fg(theme::SUCCESS),
                ));

            let inner = improvements_block.inner(chunks[chunk_idx]);
            improvements_block.render(chunks[chunk_idx], buf);

            let lines: Vec<Line> = self
                .improvements
                .iter()
                .map(|item| {
                    Line::from(vec![
                        Span::styled("• ", Style::default().fg(theme::SUCCESS)),
                        Span::styled(item.as_str(), Style::default().fg(theme::SECONDARY)),
                    ])
                })
                .collect();

            let paragraph = Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false });
            paragraph.render(inner, buf);

            chunk_idx += 1;
        }

        // Render unclear parts
        if has_unclear {
            let unclear_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme::BORDER))
                .title(Span::styled(
                    " Unclear Parts ",
                    Style::default().fg(theme::WARNING),
                ));

            let target_chunk = if has_improvements {
                chunks[chunk_idx]
            } else {
                chunks[0]
            };

            let inner = unclear_block.inner(target_chunk);
            unclear_block.render(target_chunk, buf);

            let lines: Vec<Line> = self
                .unclear_parts
                .iter()
                .map(|item| {
                    Line::from(vec![
                        Span::styled("• ", Style::default().fg(theme::WARNING)),
                        Span::styled(item.as_str(), Style::default().fg(theme::SECONDARY)),
                    ])
                })
                .collect();

            let paragraph = Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false });
            paragraph.render(inner, buf);
        }

        // If neither has content, show a message
        if !has_improvements && !has_unclear {
            let empty_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme::BORDER))
                .title(Span::styled(
                    " Feedback ",
                    Style::default().fg(theme::SECONDARY),
                ));

            let inner = empty_block.inner(area);
            empty_block.render(area, buf);

            let paragraph = Paragraph::new(Text::styled(
                "No feedback available.",
                Style::default().fg(theme::MUTED),
            ));
            paragraph.render(inner, buf);
        }
    }
}
