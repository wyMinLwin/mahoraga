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

// Padding constants
const PADDING_X: u16 = 1;
const PADDING_Y: u16 = 1;

impl<'a> Feedback<'a> {
    pub fn new(improvements: &'a [String], unclear_parts: &'a [String]) -> Self {
        Self {
            improvements,
            unclear_parts,
        }
    }

    /// Add padding to a rect
    fn with_padding(area: Rect) -> Rect {
        Rect {
            x: area.x + PADDING_X,
            y: area.y + PADDING_Y,
            width: area.width.saturating_sub(PADDING_X * 2),
            height: area.height.saturating_sub(PADDING_Y * 2),
        }
    }

    /// Calculate the height needed for a section based on content and width
    fn calculate_section_height(items: &[String], width: u16) -> u16 {
        if items.is_empty() {
            return 0;
        }
        // Account for borders (2), padding (2), and bullet (2)
        let inner_width = width.saturating_sub(4 + PADDING_X * 2 + 2) as usize;
        let mut total_lines: u16 = 0;
        for item in items {
            // Estimate wrapped lines: ceil(text_len / inner_width)
            let lines = if inner_width > 0 {
                (item.len() as f32 / inner_width as f32).ceil() as u16
            } else {
                1
            };
            total_lines += lines.max(1);
        }
        total_lines + 2 + (PADDING_Y * 2) // +2 for borders, +padding
    }

    /// Calculate total height needed for this widget
    pub fn calculate_height(&self, width: u16) -> u16 {
        let improvements_height = Self::calculate_section_height(self.improvements, width);
        let unclear_height = Self::calculate_section_height(self.unclear_parts, width);

        if improvements_height == 0 && unclear_height == 0 {
            4 // Empty state with message
        } else {
            improvements_height + unclear_height
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
            let padded = Self::with_padding(inner);
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
            paragraph.render(padded, buf);

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
            let padded = Self::with_padding(inner);
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
            paragraph.render(padded, buf);
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
            let padded = Self::with_padding(inner);
            empty_block.render(area, buf);

            let paragraph = Paragraph::new(Text::styled(
                "No feedback available.",
                Style::default().fg(theme::MUTED),
            ));
            paragraph.render(padded, buf);
        }
    }
}
