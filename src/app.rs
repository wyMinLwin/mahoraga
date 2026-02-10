use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use rand::seq::SliceRandom;
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc;

use crate::config::{load_config, reset_config, save_config};

const ANALYZING_SYNONYMS: &[&str] = &[
    "Scrutinizing",
    "Dissecting",
    "Examining",
    "Evaluating",
    "Deconstructing",
    "Appraising",
    "Assessing",
    "Investigating",
    "Elucidating",
    "Interrogating",
];
use crate::providers::create_provider;
use crate::types::{AnalysisResult, AppState, Command, Config, ProviderType, Screen, SettingsField};
use crate::ui::{MainScreen, SettingsScreen};

/// Message from async analysis task
enum AsyncMessage {
    AnalysisComplete(Result<AnalysisResult>),
}

/// Main application structure
pub struct App {
    /// Current screen
    screen: Screen,
    /// App state
    state: AppState,
    /// Current prompt text
    prompt: String,
    /// Cursor position in prompt
    cursor_position: usize,
    /// Analysis result
    result: Option<AnalysisResult>,
    /// Error message
    error: Option<String>,
    /// Should quit
    should_quit: bool,
    /// Configuration
    config: Config,
    /// Command menu selected index
    command_selected: usize,
    /// Command menu filter
    command_filter: String,
    /// Settings screen selected field
    settings_selected: usize,
    /// Settings editing mode
    settings_editing: bool,
    /// Settings edit buffer
    settings_edit_value: String,
    /// Settings edit cursor position
    settings_cursor: usize,
    /// Settings message
    settings_message: Option<String>,
    /// Settings message is error
    settings_is_error: bool,
    /// Working copy of config for settings
    settings_config: Config,
    /// Animation frame for analyzing state (0, 1, 2 = 1, 2, 3 dots)
    analyzing_animation_frame: u8,
    /// Selected synonym for analyzing message
    analyzing_word: String,
    /// Tick counter for animation timing
    animation_tick_counter: u8,
}

impl App {
    pub fn new() -> Result<Self> {
        let config = load_config()?;
        let settings_config = config.clone();

        Ok(Self {
            screen: Screen::Main,
            state: AppState::Idle,
            prompt: String::new(),
            cursor_position: 0,
            result: None,
            error: None,
            should_quit: false,
            config,
            command_selected: 0,
            command_filter: String::new(),
            settings_selected: 0,
            settings_editing: false,
            settings_edit_value: String::new(),
            settings_cursor: 0,
            settings_message: None,
            settings_is_error: false,
            settings_config,
            analyzing_animation_frame: 0,
            analyzing_word: String::new(),
            animation_tick_counter: 0,
        })
    }

    /// Run the application
    pub async fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
        let (tx, mut rx) = mpsc::channel::<AsyncMessage>(1);

        loop {
            // Draw UI
            terminal.draw(|f| self.render(f))?;

            // Check for async messages
            if let Ok(msg) = rx.try_recv() {
                match msg {
                    AsyncMessage::AnalysisComplete(result) => {
                        self.state = AppState::Idle;
                        match result {
                            Ok(analysis) => {
                                self.result = Some(analysis);
                                self.state = AppState::ShowingResults;
                                self.error = None;
                            }
                            Err(e) => {
                                self.error = Some(e.to_string());
                            }
                        }
                    }
                }
            }

            // Update animation frame when analyzing (every ~250ms = 5 ticks at 50ms)
            if self.state == AppState::Analyzing {
                self.animation_tick_counter += 1;
                if self.animation_tick_counter >= 5 {
                    self.animation_tick_counter = 0;
                    self.analyzing_animation_frame = (self.analyzing_animation_frame + 1) % 3;
                }
            }

            // Handle input events
            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    // Handle Ctrl+C globally
                    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                        self.should_quit = true;
                    } else {
                        match self.screen {
                            Screen::Main => self.handle_main_input(key, &tx).await?,
                            Screen::Settings => self.handle_settings_input(key.code)?,
                        }
                    }
                }
            }

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn render(&self, frame: &mut ratatui::Frame) {
        match self.screen {
            Screen::Main => {
                let screen = MainScreen::new(&self.prompt, self.cursor_position)
                    .result(self.result.as_ref())
                    .state(self.state)
                    .error(self.error.as_deref())
                    .command_menu(self.command_selected, self.command_filter.clone())
                    .provider(self.config.provider.active)
                    .analyzing_animation(&self.analyzing_word, self.analyzing_animation_frame);

                frame.render_widget(screen, frame.area());
            }
            Screen::Settings => {
                // Render main screen as frozen background
                let bg = MainScreen::new(&self.prompt, self.cursor_position)
                    .result(self.result.as_ref())
                    .state(self.state)
                    .error(self.error.as_deref())
                    .command_menu(self.command_selected, self.command_filter.clone())
                    .provider(self.config.provider.active)
                    .analyzing_animation(&self.analyzing_word, self.analyzing_animation_frame);
                frame.render_widget(bg, frame.area());

                // Overlay settings popup on top
                let screen = SettingsScreen::new(&self.settings_config)
                    .selected(self.settings_selected)
                    .editing(self.settings_editing, &self.settings_edit_value, self.settings_cursor)
                    .message(self.settings_message.as_deref(), self.settings_is_error);

                frame.render_widget(screen, frame.area());
            }
        }
    }

    async fn handle_main_input(&mut self, key: KeyEvent, tx: &mpsc::Sender<AsyncMessage>) -> Result<()> {
        match self.state {
            AppState::Analyzing => {
                // Can only cancel with Escape
                if key.code == KeyCode::Esc {
                    self.state = AppState::Idle;
                }
            }
            _ => self.handle_prompt_input(key, tx).await?,
        }
        Ok(())
    }

    async fn handle_prompt_input(&mut self, key: KeyEvent, tx: &mpsc::Sender<AsyncMessage>) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                if self.state == AppState::CommandMenu {
                    self.state = AppState::Idle;
                    self.command_filter.clear();
                } else if self.state == AppState::ShowingResults {
                    self.state = AppState::Idle;
                }
            }
            KeyCode::Enter => {
                if self.prompt.starts_with('/') {
                    // Try to execute as command
                    let cmd = if self.state == AppState::CommandMenu {
                        // Use selected command from menu
                        self.filtered_commands().get(self.command_selected).copied()
                    } else {
                        // Find exact match
                        Command::all().iter().find(|c| c.name() == self.prompt).copied()
                    };

                    if let Some(cmd) = cmd {
                        self.execute_command(cmd);
                        self.prompt.clear();
                        self.cursor_position = 0;
                    }
                    self.state = AppState::Idle;
                    self.command_filter.clear();
                } else if !self.prompt.is_empty() && self.state != AppState::Analyzing {
                    // Not a command, analyze
                    self.start_analysis(tx.clone()).await;
                }
            }
            KeyCode::Char(c) => {
                // Ctrl+U: delete from cursor to start of line
                if c == 'u' && key.modifiers.contains(KeyModifiers::CONTROL) {
                    if self.cursor_position > 0 {
                        let before_cursor = &self.prompt[..self.cursor_position];
                        let line_start = before_cursor.rfind('\n').map(|i| i + 1).unwrap_or(0);
                        self.prompt.drain(line_start..self.cursor_position);
                        self.cursor_position = line_start;
                        self.error = None;

                        // Update command menu state
                        if self.prompt.starts_with('/') {
                            self.command_filter = self.prompt.clone();
                            self.command_selected = 0;
                            if self.filtered_commands().is_empty() {
                                self.state = AppState::Idle;
                                self.command_filter.clear();
                            } else {
                                self.state = AppState::CommandMenu;
                            }
                        } else {
                            self.state = AppState::Idle;
                            self.command_filter.clear();
                        }
                    }
                    return Ok(());
                }

                self.prompt.insert(self.cursor_position, c);
                self.cursor_position += 1;
                self.error = None;

                // Show command menu when prompt starts with /
                if self.prompt.starts_with('/') {
                    self.command_filter = self.prompt.clone();
                    self.command_selected = 0;
                    // Hide if no matching commands
                    if self.filtered_commands().is_empty() {
                        self.state = AppState::Idle;
                        self.command_filter.clear();
                    } else {
                        self.state = AppState::CommandMenu;
                    }
                } else if self.state == AppState::CommandMenu {
                    self.state = AppState::Idle;
                    self.command_filter.clear();
                }
            }
            KeyCode::Tab => {
                // Insert selected command and hide menu
                if self.state == AppState::CommandMenu {
                    let commands = self.filtered_commands();
                    if let Some(cmd) = commands.get(self.command_selected) {
                        self.prompt = cmd.name().to_string();
                        self.cursor_position = self.prompt.len();
                    }
                    self.state = AppState::Idle;
                    self.command_filter.clear();
                }
            }
            KeyCode::Backspace => {
                if self.cursor_position > 0 {
                    // Normal backspace: delete one character
                    self.cursor_position -= 1;
                    self.prompt.remove(self.cursor_position);
                    self.error = None;

                    // Update command menu state
                    if self.prompt.starts_with('/') {
                        self.command_filter = self.prompt.clone();
                        self.command_selected = 0;
                        // Hide if no matching commands
                        if self.filtered_commands().is_empty() {
                            self.state = AppState::Idle;
                            self.command_filter.clear();
                        } else {
                            self.state = AppState::CommandMenu;
                        }
                    } else {
                        self.state = AppState::Idle;
                        self.command_filter.clear();
                    }
                }
            }
            KeyCode::Delete => {
                if self.cursor_position < self.prompt.len() {
                    self.prompt.remove(self.cursor_position);
                    self.error = None;
                }
            }
            KeyCode::Left => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_position < self.prompt.len() {
                    self.cursor_position += 1;
                }
            }
            KeyCode::Up => {
                if self.state == AppState::CommandMenu && self.command_selected > 0 {
                    self.command_selected -= 1;
                }
            }
            KeyCode::Down => {
                if self.state == AppState::CommandMenu {
                    let commands = self.filtered_commands();
                    if self.command_selected < commands.len().saturating_sub(1) {
                        self.command_selected += 1;
                    }
                }
            }
            KeyCode::Home => {
                self.cursor_position = 0;
            }
            KeyCode::End => {
                self.cursor_position = self.prompt.len();
            }
            _ => {}
        }
        Ok(())
    }

    fn filtered_commands(&self) -> Vec<Command> {
        let filter_lower = self.command_filter.to_lowercase();
        Command::all()
            .iter()
            .filter(|cmd| {
                if filter_lower.is_empty() || filter_lower == "/" {
                    true
                } else {
                    cmd.name().to_lowercase().contains(&filter_lower)
                        || cmd.description().to_lowercase().contains(&filter_lower)
                }
            })
            .copied()
            .collect()
    }

    fn execute_command(&mut self, cmd: Command) {
        match cmd {
            Command::Settings => {
                self.settings_config = self.config.clone();
                self.settings_selected = 0;
                self.settings_editing = false;
                self.settings_message = None;
                self.screen = Screen::Settings;
            }
            Command::Clear => {
                self.prompt.clear();
                self.cursor_position = 0;
                self.result = None;
                self.error = None;
                self.state = AppState::Idle;
            }
            Command::Exit => {
                self.should_quit = true;
            }
            Command::Provider => {
                // Cycle through providers
                self.config.provider.active = self.config.provider.active.next();
                if let Err(e) = save_config(&self.config) {
                    self.error = Some(format!("Failed to save config: {}", e));
                }
            }
            Command::Default => {
                match reset_config() {
                    Ok(config) => {
                        self.config = config;
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(format!("Failed to reset config: {}", e));
                    }
                }
            }
        }
    }

    async fn start_analysis(&mut self, tx: mpsc::Sender<AsyncMessage>) {
        self.state = AppState::Analyzing;
        self.result = None; // Clear old results
        self.error = None;

        // Select random synonym and reset animation
        let mut rng = rand::thread_rng();
        self.analyzing_word = ANALYZING_SYNONYMS
            .choose(&mut rng)
            .unwrap_or(&"Analyzing")
            .to_string();
        self.analyzing_animation_frame = 0;
        self.animation_tick_counter = 0;

        let config = self.config.clone();
        let prompt = self.prompt.clone();

        tokio::spawn(async move {
            let result = async {
                let provider = create_provider(&config)?;
                provider.analyze(&prompt).await
            }
            .await;

            let _ = tx.send(AsyncMessage::AnalysisComplete(result)).await;
        });
    }

    fn handle_settings_input(&mut self, key: KeyCode) -> Result<()> {
        let fields = SettingsField::fields_for_provider(self.settings_config.provider.active);

        if self.settings_editing {
            self.handle_settings_edit_input(key, &fields)?;
        } else {
            self.handle_settings_nav_input(key, &fields)?;
        }

        Ok(())
    }

    fn handle_settings_nav_input(&mut self, key: KeyCode, fields: &[SettingsField]) -> Result<()> {
        match key {
            KeyCode::Esc => {
                self.screen = Screen::Main;
            }
            KeyCode::Up => {
                if self.settings_selected > 0 {
                    self.settings_selected -= 1;
                    self.settings_message = None;
                }
            }
            KeyCode::Down | KeyCode::Tab => {
                if self.settings_selected < fields.len() - 1 {
                    self.settings_selected += 1;
                    self.settings_message = None;
                }
            }
            KeyCode::Left | KeyCode::Right => {
                if let Some(field) = fields.get(self.settings_selected) {
                    if field.is_provider_selector() {
                        // Cycle provider
                        if key == KeyCode::Right {
                            self.settings_config.provider.active = self.settings_config.provider.active.next();
                        } else {
                            // Cycle backwards
                            self.settings_config.provider.active = match self.settings_config.provider.active {
                                ProviderType::Azure => ProviderType::Anthropic,
                                ProviderType::OpenAI => ProviderType::Azure,
                                ProviderType::Anthropic => ProviderType::OpenAI,
                            };
                        }
                        // Reset selected field when provider changes
                        self.settings_selected = 0;
                    }
                }
            }
            KeyCode::Enter => {
                if let Some(field) = fields.get(self.settings_selected) {
                    if *field == SettingsField::Save {
                        // Save configuration
                        self.config = self.settings_config.clone();
                        if let Err(e) = save_config(&self.config) {
                            self.settings_message = Some(format!("Error: {}", e));
                            self.settings_is_error = true;
                        } else {
                            self.settings_message = Some("Settings saved!".to_string());
                            self.settings_is_error = false;
                            self.screen = Screen::Main;
                        }
                    } else if *field == SettingsField::Cancel {
                        self.screen = Screen::Main;
                    } else if !field.is_button() && !field.is_provider_selector() {
                        // Start editing
                        self.settings_editing = true;
                        self.settings_edit_value = self.get_settings_field_value(field);
                        self.settings_cursor = self.settings_edit_value.len();
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_settings_edit_input(&mut self, key: KeyCode, fields: &[SettingsField]) -> Result<()> {
        match key {
            KeyCode::Esc => {
                self.settings_editing = false;
                self.settings_edit_value.clear();
            }
            KeyCode::Enter => {
                // Save the edited value
                if let Some(field) = fields.get(self.settings_selected) {
                    self.set_settings_field_value(field, self.settings_edit_value.clone());
                }
                self.settings_editing = false;
                self.settings_edit_value.clear();
            }
            KeyCode::Char(c) => {
                self.settings_edit_value.insert(self.settings_cursor, c);
                self.settings_cursor += 1;
            }
            KeyCode::Backspace => {
                if self.settings_cursor > 0 {
                    self.settings_cursor -= 1;
                    self.settings_edit_value.remove(self.settings_cursor);
                }
            }
            KeyCode::Delete => {
                if self.settings_cursor < self.settings_edit_value.len() {
                    self.settings_edit_value.remove(self.settings_cursor);
                }
            }
            KeyCode::Left => {
                if self.settings_cursor > 0 {
                    self.settings_cursor -= 1;
                }
            }
            KeyCode::Right => {
                if self.settings_cursor < self.settings_edit_value.len() {
                    self.settings_cursor += 1;
                }
            }
            KeyCode::Home => {
                self.settings_cursor = 0;
            }
            KeyCode::End => {
                self.settings_cursor = self.settings_edit_value.len();
            }
            _ => {}
        }
        Ok(())
    }

    fn get_settings_field_value(&self, field: &SettingsField) -> String {
        match field {
            SettingsField::AzureUrl => self.settings_config.azure.url.clone(),
            SettingsField::AzureApiKey => self.settings_config.azure.api_key.clone(),
            SettingsField::AzureDeployment => self.settings_config.azure.deployment.clone(),
            SettingsField::AzureApiVersion => self.settings_config.azure.api_version.clone(),
            SettingsField::OpenAIApiKey => self.settings_config.openai.api_key.clone(),
            SettingsField::OpenAIModel => self.settings_config.openai.model.clone(),
            SettingsField::AnthropicApiKey => self.settings_config.anthropic.api_key.clone(),
            SettingsField::AnthropicModel => self.settings_config.anthropic.model.clone(),
            _ => String::new(),
        }
    }

    fn set_settings_field_value(&mut self, field: &SettingsField, value: String) {
        match field {
            SettingsField::AzureUrl => self.settings_config.azure.url = value,
            SettingsField::AzureApiKey => self.settings_config.azure.api_key = value,
            SettingsField::AzureDeployment => self.settings_config.azure.deployment = value,
            SettingsField::AzureApiVersion => self.settings_config.azure.api_version = value,
            SettingsField::OpenAIApiKey => self.settings_config.openai.api_key = value,
            SettingsField::OpenAIModel => self.settings_config.openai.model = value,
            SettingsField::AnthropicApiKey => self.settings_config.anthropic.api_key = value,
            SettingsField::AnthropicModel => self.settings_config.anthropic.model = value,
            _ => {}
        }
    }
}
