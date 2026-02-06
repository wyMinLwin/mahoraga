use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
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
    analyzing_word: &'a str,
    analyzing_frame: u8,
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
            analyzing_word: "",
            analyzing_frame: 0,
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

    pub fn analyzing_animation(mut self, word: &'a str, frame: u8) -> Self {
        self.analyzing_word = word;
        self.analyzing_frame = frame;
        self
    }
}

impl Widget for MainScreen<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Determine if we have enough space for the full logo
        let show_logo = area.height >= 22;
        let header_height = Header::height(show_logo);

        // Layout calculations
        let show_command_menu = self.state == AppState::CommandMenu;
        let is_analyzing = self.state == AppState::Analyzing;

        // Calculate dynamic prompt height based on newline count
        let line_count = self.prompt.matches('\n').count() + 1;
        let prompt_height = (line_count as u16) + 2; // +2 for borders

        let mut constraints = vec![
            Constraint::Length(header_height),
            Constraint::Length(2), // Provider indicator + margin bottom
        ];

        // Analyzing indicator (above input box)
        if is_analyzing {
            constraints.push(Constraint::Length(1));
        }

        // Results above input (if available)
        if let Some(result) = &self.result {
            constraints.push(Constraint::Length(4)); // Score display (fixed height)
            // Calculate feedback height based on content
            let feedback_width = 60.min(area.width.saturating_sub(2)); // Match fixed width
            let feedback = Feedback::new(&result.improvements, &result.unclear_parts);
            let feedback_height = feedback.calculate_height(feedback_width);
            constraints.push(Constraint::Length(feedback_height));
        }

        // Prompt input always at the end
        constraints.push(Constraint::Length(prompt_height.max(3)));

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

        // Analyzing indicator (above input box)
        if is_analyzing {
            let dots = ".".repeat((self.analyzing_frame + 1) as usize);
            let analyzing_text = format!("{}{}", self.analyzing_word, dots);
            let analyzing_line = Line::from(Span::styled(
                analyzing_text,
                Style::default()
                    .fg(theme::PRIMARY)
                    .add_modifier(Modifier::ITALIC),
            ));
            Paragraph::new(analyzing_line).render(chunks[chunk_idx], buf);
            chunk_idx += 1;
        }

        // Results above input (if available)
        if let Some(result) = self.result {
            // Score display with fixed width
            let score_area = chunks[chunk_idx];
            let fixed_score_area = Rect {
                x: score_area.x,
                y: score_area.y,
                width: 60.min(score_area.width), // Fixed width of 60
                height: score_area.height,
            };
            ScoreDisplay::new(result.score).render(fixed_score_area, buf);
            chunk_idx += 1;

            // Feedback with fixed width
            let feedback_area = chunks[chunk_idx];
            let fixed_feedback_area = Rect {
                x: feedback_area.x,
                y: feedback_area.y,
                width: 60.min(feedback_area.width), // Fixed width of 60
                height: feedback_area.height,
            };
            Feedback::new(&result.improvements, &result.unclear_parts)
                .render(fixed_feedback_area, buf);
            chunk_idx += 1;
        }

        // Prompt input
        PromptInput::new(self.prompt, self.cursor_position)
            .focused(self.state != AppState::CommandMenu)
            .render(chunks[chunk_idx], buf);
        let prompt_chunk = chunks[chunk_idx];
        chunk_idx += 1;

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
