use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::types::{AnalysisResult, AppState, ProviderType};
use crate::ui::theme;
use crate::ui::widgets::{CommandMenu, Feedback, Header, PromptInput, ScoreDisplay};

pub struct MainScreen<'a> {
    prompt: &'a str,
    cursor_position: usize,
    result: Option<&'a AnalysisResult>,
    state: AppState,
    error: Option<&'a str>,
    command_selected: usize,
    command_filter: String,
    active_provider: ProviderType,
}

impl<'a> MainScreen<'a> {
    pub fn new(prompt: &'a str, cursor_position: usize) -> Self {
        Self {
            prompt,
            cursor_position,
            result: None,
            state: AppState::Idle,
            error: None,
            command_selected: 0,
            command_filter: String::new(),
            active_provider: ProviderType::Azure,
        }
    }

    pub fn result(mut self, result: Option<&'a AnalysisResult>) -> Self {
        self.result = result;
        self
    }

    pub fn state(mut self, state: AppState) -> Self {
        self.state = state;
        self
    }

    pub fn error(mut self, error: Option<&'a str>) -> Self {
        self.error = error;
        self
    }

    pub fn command_menu(mut self, selected: usize, filter: String) -> Self {
        self.command_selected = selected;
        self.command_filter = filter;
        self
    }

    pub fn provider(mut self, provider: ProviderType) -> Self {
        self.active_provider = provider;
        self
    }
}

impl Widget for MainScreen<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Determine if we have enough space for the full logo
        let show_logo = area.height >= 25;
        let header_height = Header::height(show_logo);

        // Layout calculations
        let has_result = self.result.is_some();
        let show_command_menu = self.state == AppState::CommandMenu;

        // Calculate dynamic prompt height based on newline count
        let line_count = self.prompt.matches('\n').count() + 1;
        let prompt_height = (line_count as u16) + 2; // +2 for borders

        let mut constraints = vec![
            Constraint::Length(header_height),
            Constraint::Length(1), // Provider indicator
        ];

        if has_result {
            constraints.push(Constraint::Length(prompt_height.max(3))); // Prompt input
            constraints.push(Constraint::Length(4)); // Score display
            constraints.push(Constraint::Min(8));    // Feedback
        } else {
            constraints.push(Constraint::Length(prompt_height.max(3))); // Prompt input (min 3 = 1 line + borders)
        }

        // Error bar (only shown when there's an error)
        if self.error.is_some() {
            constraints.push(Constraint::Length(1));
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(constraints)
            .split(area);

        let mut chunk_idx = 0;

        // Header
        Header::new(show_logo).render(chunks[chunk_idx], buf);
        chunk_idx += 1;

        // Provider indicator
        let provider_line = Line::from(vec![
            Span::styled("Provider: ", Style::default().fg(theme::MUTED)),
            Span::styled(
                self.active_provider.display_name(),
                Style::default().fg(theme::PRIMARY),
            ),
        ]);
        Paragraph::new(provider_line).render(chunks[chunk_idx], buf);
        chunk_idx += 1;

        // Prompt input
        let is_analyzing = self.state == AppState::Analyzing;
        PromptInput::new(self.prompt, self.cursor_position)
            .focused(self.state != AppState::CommandMenu)
            .analyzing(is_analyzing)
            .render(chunks[chunk_idx], buf);
        let prompt_chunk = chunks[chunk_idx];
        chunk_idx += 1;

        // Results (if available)
        if let Some(result) = self.result {
            // Score display
            ScoreDisplay::new(result.score).render(chunks[chunk_idx], buf);
            chunk_idx += 1;

            // Feedback
            Feedback::new(&result.improvements, &result.unclear_parts)
                .render(chunks[chunk_idx], buf);
            chunk_idx += 1;
        }

        // Error bar (only shown when there's an error)
        if let Some(err) = self.error {
            let error_line = Line::from(vec![
                Span::styled("Error: ", Style::default().fg(theme::ERROR)),
                Span::styled(err, Style::default().fg(theme::ERROR)),
            ]);
            Paragraph::new(error_line).render(chunks[chunk_idx], buf);
        }

        // Command menu overlay
        if show_command_menu {
            let menu = CommandMenu::new(self.command_selected, self.command_filter.clone());
            let menu_height = menu.height();

            // Position menu below the prompt input
            let menu_y = (prompt_chunk.y + prompt_chunk.height).min(area.height.saturating_sub(menu_height + 1));
            let menu_width = 50.min(area.width.saturating_sub(4));
            let menu_x = prompt_chunk.x + 1;

            let menu_area = Rect::new(menu_x, menu_y, menu_width, menu_height);
            menu.render(menu_area, buf);
        }
    }
}
