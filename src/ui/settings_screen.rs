use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use crate::types::{Config, SettingsField};
use crate::ui::theme;

/// Returns a centered `Rect` of `width` x `height` within `outer`.
fn centered_rect(width: u16, height: u16, outer: Rect) -> Rect {
    let x = outer.x + outer.width.saturating_sub(width) / 2;
    let y = outer.y + outer.height.saturating_sub(height) / 2;
    Rect::new(x, y, width.min(outer.width), height.min(outer.height))
}

pub struct SettingsScreen<'a> {
    config: &'a Config,
    selected_field: usize,
    editing: bool,
    edit_value: &'a str,
    cursor_position: usize,
    message: Option<&'a str>,
    is_error: bool,
}

impl<'a> SettingsScreen<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            selected_field: 0,
            editing: false,
            edit_value: "",
            cursor_position: 0,
            message: None,
            is_error: false,
        }
    }

    pub fn selected(mut self, field: usize) -> Self {
        self.selected_field = field;
        self
    }

    pub fn editing(mut self, editing: bool, value: &'a str, cursor: usize) -> Self {
        self.editing = editing;
        self.edit_value = value;
        self.cursor_position = cursor;
        self
    }

    pub fn message(mut self, msg: Option<&'a str>, is_error: bool) -> Self {
        self.message = msg;
        self.is_error = is_error;
        self
    }

    fn get_field_value(&self, field: &SettingsField) -> String {
        match field {
            SettingsField::Provider => self.config.provider.active.display_name().to_string(),
            SettingsField::AzureUrl => self.config.azure.url.clone(),
            SettingsField::AzureApiKey => {
                if self.config.azure.api_key.is_empty() {
                    String::new()
                } else {
                    "•".repeat(self.config.azure.api_key.len().min(20))
                }
            }
            SettingsField::AzureDeployment => self.config.azure.deployment.clone(),
            SettingsField::AzureApiVersion => self.config.azure.api_version.clone(),
            SettingsField::OpenAIApiKey => {
                if self.config.openai.api_key.is_empty() {
                    String::new()
                } else {
                    "•".repeat(self.config.openai.api_key.len().min(20))
                }
            }
            SettingsField::OpenAIModel => self.config.openai.model.clone(),
            SettingsField::AnthropicApiKey => {
                if self.config.anthropic.api_key.is_empty() {
                    String::new()
                } else {
                    "•".repeat(self.config.anthropic.api_key.len().min(20))
                }
            }
            SettingsField::AnthropicModel => self.config.anthropic.model.clone(),
            SettingsField::Save | SettingsField::Cancel => String::new(),
        }
    }
}

impl Widget for SettingsScreen<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Get fields for current provider
        let fields = SettingsField::fields_for_provider(self.config.provider.active);

        // Compute popup dimensions dynamically
        let content_height: u16 = fields
            .iter()
            .map(|f| if f.is_button() { 1u16 } else { 2u16 })
            .sum();
        // borders(2) + status bar(1) + padding(1)
        let popup_height = (content_height + 4).min(area.height);
        let popup_width = 60u16.min(area.width.saturating_sub(4));

        let popup_area = centered_rect(popup_width, popup_height, area);

        // Clear only the popup area
        Clear.render(popup_area, buf);

        // Split popup into form + status bar
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(popup_area);

        // Settings form block
        let form_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme::PRIMARY))
            .title(Span::styled(
                " Settings ",
                Style::default().fg(theme::PRIMARY),
            ));

        let form_inner = form_block.inner(chunks[0]);
        form_block.render(chunks[0], buf);

        // Calculate layout for fields
        let field_constraints: Vec<Constraint> = fields
            .iter()
            .map(|f| {
                if f.is_button() {
                    Constraint::Length(1)
                } else {
                    Constraint::Length(2)
                }
            })
            .collect();

        let field_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(field_constraints)
            .split(form_inner);

        for (idx, field) in fields.iter().enumerate() {
            let is_selected = idx == self.selected_field;

            if field.is_button() {
                // Render button
                let style = if is_selected {
                    Style::default()
                        .fg(theme::BACKGROUND)
                        .bg(theme::PRIMARY)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(theme::SECONDARY)
                };

                let button_text = format!("[ {} ]", field.label());
                let line = Line::styled(button_text, style);
                Paragraph::new(line).render(field_chunks[idx], buf);
            } else if field.is_provider_selector() {
                // Render provider selector
                let label_style = if is_selected {
                    Style::default().fg(theme::PRIMARY)
                } else {
                    Style::default().fg(theme::SECONDARY)
                };

                let value = self.config.provider.active.display_name();
                let hint = if is_selected { " (← →)" } else { "" };

                let lines = vec![
                    Line::from(vec![
                        Span::styled(format!("{}: ", field.label()), label_style),
                        Span::styled(value, Style::default().fg(theme::PRIMARY)),
                        Span::styled(hint, Style::default().fg(theme::MUTED)),
                    ]),
                ];
                Paragraph::new(Text::from(lines)).render(field_chunks[idx], buf);
            } else {
                // Render text field
                let label_style = if is_selected {
                    Style::default().fg(theme::PRIMARY)
                } else {
                    Style::default().fg(theme::SECONDARY)
                };

                let value = if self.editing && is_selected {
                    if field.is_password() {
                        "•".repeat(self.edit_value.len())
                    } else {
                        self.edit_value.to_string()
                    }
                } else {
                    self.get_field_value(field)
                };

                let value_style = if is_selected && self.editing {
                    Style::default().fg(theme::PRIMARY).add_modifier(Modifier::UNDERLINED)
                } else {
                    Style::default().fg(theme::SECONDARY)
                };

                let cursor_hint = if is_selected && !self.editing {
                    " (Enter to edit)"
                } else {
                    ""
                };

                let lines = vec![
                    Line::from(vec![
                        Span::styled(format!("{}: ", field.label()), label_style),
                    ]),
                    Line::from(vec![
                        Span::raw("  "),
                        Span::styled(
                            if value.is_empty() { "(empty)" } else { &value },
                            if value.is_empty() {
                                Style::default().fg(theme::MUTED)
                            } else {
                                value_style
                            },
                        ),
                        Span::styled(cursor_hint, Style::default().fg(theme::MUTED)),
                    ]),
                ];

                Paragraph::new(Text::from(lines)).render(field_chunks[idx], buf);
            }
        }

        // Status bar (below the form block)
        let status_text = if let Some(msg) = self.message {
            let color = if self.is_error { theme::ERROR } else { theme::SUCCESS };
            Line::styled(msg, Style::default().fg(color))
        } else if self.editing {
            Line::styled(
                "Enter to save | Esc to cancel",
                Style::default().fg(theme::MUTED),
            )
        } else {
            Line::styled(
                "↑↓ Navigate | Enter to edit | Tab to next | Esc to close",
                Style::default().fg(theme::MUTED),
            )
        };

        Paragraph::new(status_text).render(chunks[1], buf);
    }
}
