use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::ui::theme;

pub struct PromptInput<'a> {
    content: &'a str,
    cursor_position: usize,
    is_focused: bool,
    is_analyzing: bool,
}

impl<'a> PromptInput<'a> {
    pub fn new(content: &'a str, cursor_position: usize) -> Self {
        Self {
            content,
            cursor_position,
            is_focused: true,
            is_analyzing: false,
        }
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.is_focused = focused;
        self
    }

    pub fn analyzing(mut self, analyzing: bool) -> Self {
        self.is_analyzing = analyzing;
        self
    }
}

impl Widget for PromptInput<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_color = if self.is_focused {
            theme::PRIMARY
        } else {
            theme::BORDER
        };

        let title = if self.is_analyzing {
            " Analyzing... "
        } else {
            ""
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title(Span::styled(title, Style::default().fg(theme::SECONDARY)));

        let inner_area = block.inner(area);
        block.render(area, buf);

        // Add padding (1 char from left)
        let padded_area = Rect {
            x: inner_area.x + 1,
            y: inner_area.y,
            width: inner_area.width.saturating_sub(2),
            height: inner_area.height,
        };

        // Show placeholder with cursor when empty
        if self.content.is_empty() {
            if self.is_focused && !self.is_analyzing {
                // Show cursor at start position
                let cursor_span = Span::styled(
                    " ",
                    Style::default()
                        .fg(theme::BACKGROUND)
                        .bg(theme::PRIMARY)
                        .add_modifier(Modifier::BOLD),
                );
                let placeholder_span = Span::styled(
                    "With this treasure, I summon Eight-Handled Sword Divergent Sila Divine General Mahoraga",
                    Style::default().fg(theme::MUTED),
                );
                let line = Line::from(vec![cursor_span, placeholder_span]);
                Paragraph::new(line).render(padded_area, buf);
            } else {
                let placeholder = Paragraph::new(Text::styled(
                    "Enter your prompt (Enter to analyze)",
                    Style::default().fg(theme::MUTED),
                ));
                placeholder.render(padded_area, buf);
            }
            return;
        }

        // Render content with cursor (supports multi-line)
        if self.is_focused && !self.is_analyzing {
            let lines: Vec<&str> = self.content.split('\n').collect();
            let mut rendered_lines: Vec<Line> = Vec::new();
            let mut char_count = 0;

            for (line_idx, line_content) in lines.iter().enumerate() {
                let line_start = char_count;
                let line_end = char_count + line_content.len();

                // Check if cursor is on this line
                let cursor_in_line = self.cursor_position >= line_start
                    && (self.cursor_position <= line_end
                        || (line_idx == lines.len() - 1 && self.cursor_position == line_end));

                if cursor_in_line {
                    let cursor_offset = self.cursor_position - line_start;
                    let before_cursor = &line_content[..cursor_offset.min(line_content.len())];
                    let at_cursor = line_content
                        .chars()
                        .nth(cursor_offset)
                        .map(|c| c.to_string())
                        .unwrap_or_else(|| " ".to_string());
                    let after_cursor = if cursor_offset < line_content.len() {
                        &line_content[cursor_offset + at_cursor.len()..]
                    } else {
                        ""
                    };

                    let spans = vec![
                        Span::styled(before_cursor, Style::default().fg(theme::SECONDARY)),
                        Span::styled(
                            at_cursor,
                            Style::default()
                                .fg(theme::BACKGROUND)
                                .bg(theme::PRIMARY)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(after_cursor, Style::default().fg(theme::SECONDARY)),
                    ];
                    rendered_lines.push(Line::from(spans));
                } else {
                    rendered_lines.push(Line::from(Span::styled(
                        *line_content,
                        Style::default().fg(theme::SECONDARY),
                    )));
                }

                // Account for newline character (except for last line)
                char_count = line_end + 1;
            }

            let text = Text::from(rendered_lines);
            let paragraph = Paragraph::new(text);
            paragraph.render(padded_area, buf);
        } else {
            // Split by newlines for proper multi-line display
            let lines: Vec<Line> = self
                .content
                .split('\n')
                .map(|line| Line::from(Span::styled(line, Style::default().fg(theme::SECONDARY))))
                .collect();
            let text = Text::from(lines);
            let paragraph = Paragraph::new(text);
            paragraph.render(padded_area, buf);
        }
    }
}
