use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use crate::types::{Config, ProviderType, SettingsField};
use crate::ui::theme;
use crate::ui::widgets::Header;

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
        // Clear the screen
        Clear.render(area, buf);

        let show_logo = area.height >= 25;
        let header_height = Header::height(show_logo);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(header_height),
                Constraint::Min(10),
                Constraint::Length(1),
            ])
            .split(area);

        // Header
        Header::new(show_logo).render(chunks[0], buf);

        // Settings form
        let form_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme::PRIMARY))
            .title(Span::styled(
                " Settings ",
                Style::default().fg(theme::PRIMARY),
            ));

        let form_inner = form_block.inner(chunks[1]);
        form_block.render(chunks[1], buf);

        // Get fields for current provider
        let fields = SettingsField::fields_for_provider(self.config.provider.active);

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

        // Status bar
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

        Paragraph::new(status_text).render(chunks[2], buf);
    }
}
