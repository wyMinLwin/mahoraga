use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use crate::types::Command;
use crate::ui::theme;

pub struct CommandMenu {
    selected: usize,
    filter: String,
}

impl CommandMenu {
    pub fn new(selected: usize, filter: String) -> Self {
        Self { selected, filter }
    }

    pub fn filtered_commands(&self) -> Vec<Command> {
        let filter_lower = self.filter.to_lowercase();
        Command::all()
            .iter()
            .filter(|cmd| {
                if filter_lower.is_empty() {
                    true
                } else {
                    cmd.name().to_lowercase().contains(&filter_lower)
                        || cmd.description().to_lowercase().contains(&filter_lower)
                }
            })
            .copied()
            .collect()
    }
}

impl Widget for CommandMenu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Clear the area first
        Clear.render(area, buf);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme::PRIMARY))
            .title(Span::styled(
                " Commands ",
                Style::default().fg(theme::PRIMARY),
            ));

        let inner = block.inner(area);
        block.render(area, buf);

        let commands = self.filtered_commands();

        let lines: Vec<Line> = commands
            .iter()
            .enumerate()
            .map(|(idx, cmd)| {
                let is_selected = idx == self.selected;
                let (name_style, desc_style) = if is_selected {
                    (
                        Style::default()
                            .fg(theme::BACKGROUND)
                            .bg(theme::PRIMARY)
                            .add_modifier(Modifier::BOLD),
                        Style::default()
                            .fg(theme::BACKGROUND)
                            .bg(theme::PRIMARY),
                    )
                } else {
                    (
                        Style::default().fg(theme::PRIMARY),
                        Style::default().fg(theme::MUTED),
                    )
                };

                Line::from(vec![
                    Span::styled(format!("{:<12}", cmd.name()), name_style),
                    Span::styled(format!(" {}", cmd.description()), desc_style),
                ])
            })
            .collect();

        if lines.is_empty() {
            let no_match = Paragraph::new(Text::styled(
                "No matching commands",
                Style::default().fg(theme::MUTED),
            ));
            no_match.render(inner, buf);
        } else {
            let paragraph = Paragraph::new(Text::from(lines));
            paragraph.render(inner, buf);
        }
    }
}

impl CommandMenu {
    /// Calculate the height needed for the command menu
    pub fn height(&self) -> u16 {
        let commands = self.filtered_commands();
        (commands.len().max(1) + 2) as u16 // +2 for borders
    }
}
