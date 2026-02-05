use serde::{Deserialize, Serialize};

/// The active LLM provider
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    #[default]
    Azure,
    OpenAI,
    Anthropic,
}

impl ProviderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProviderType::Azure => "azure",
            ProviderType::OpenAI => "openai",
            ProviderType::Anthropic => "anthropic",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            ProviderType::Azure => "Azure OpenAI",
            ProviderType::OpenAI => "OpenAI",
            ProviderType::Anthropic => "Anthropic",
        }
    }

    pub fn all() -> &'static [ProviderType] {
        &[ProviderType::Azure, ProviderType::OpenAI, ProviderType::Anthropic]
    }

    pub fn next(&self) -> ProviderType {
        match self {
            ProviderType::Azure => ProviderType::OpenAI,
            ProviderType::OpenAI => ProviderType::Anthropic,
            ProviderType::Anthropic => ProviderType::Azure,
        }
    }
}

impl std::fmt::Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Azure OpenAI configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AzureConfig {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub api_key: String,
    #[serde(default)]
    pub deployment: String,
    #[serde(default = "default_azure_api_version")]
    pub api_version: String,
}

fn default_azure_api_version() -> String {
    "2024-02-15-preview".to_string()
}

/// OpenAI configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenAIConfig {
    #[serde(default)]
    pub api_key: String,
    #[serde(default = "default_openai_model")]
    pub model: String,
}

fn default_openai_model() -> String {
    "gpt-4".to_string()
}

/// Anthropic configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnthropicConfig {
    #[serde(default)]
    pub api_key: String,
    #[serde(default = "default_anthropic_model")]
    pub model: String,
}

fn default_anthropic_model() -> String {
    "claude-sonnet-4-20250514".to_string()
}

/// Provider selection configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProviderSelection {
    #[serde(default)]
    pub active: ProviderType,
}

/// Main configuration structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub provider: ProviderSelection,
    #[serde(default)]
    pub azure: AzureConfig,
    #[serde(default)]
    pub openai: OpenAIConfig,
    #[serde(default)]
    pub anthropic: AnthropicConfig,
}

/// Result of prompt analysis from LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// Quality score from 0 to 100
    pub score: u8,
    /// List of improvement suggestions
    pub improvements: Vec<String>,
    /// List of unclear parts in the prompt
    pub unclear_parts: Vec<String>,
}

impl Default for AnalysisResult {
    fn default() -> Self {
        Self {
            score: 0,
            improvements: Vec::new(),
            unclear_parts: Vec::new(),
        }
    }
}

/// Application screen states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screen {
    #[default]
    Main,
    Settings,
}

/// Application states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppState {
    #[default]
    Idle,
    Analyzing,
    ShowingResults,
    CommandMenu,
}

/// Available commands
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Settings,
    Clear,
    Exit,
    Provider,
    Default,
}

impl Command {
    pub fn all() -> &'static [Command] {
        &[
            Command::Settings,
            Command::Provider,
            Command::Clear,
            Command::Default,
            Command::Exit,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Command::Settings => "/settings",
            Command::Clear => "/clear",
            Command::Exit => "/exit",
            Command::Provider => "/provider",
            Command::Default => "/default",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Command::Settings => "Configure API settings",
            Command::Clear => "Clear current prompt",
            Command::Exit => "Exit the application",
            Command::Provider => "Switch active provider",
            Command::Default => "Reset to default settings",
        }
    }
}

/// Settings screen field selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsField {
    Provider,
    // Azure fields
    AzureUrl,
    AzureApiKey,
    AzureDeployment,
    AzureApiVersion,
    // OpenAI fields
    OpenAIApiKey,
    OpenAIModel,
    // Anthropic fields
    AnthropicApiKey,
    AnthropicModel,
    // Action buttons
    Save,
    Cancel,
}

impl SettingsField {
    /// Get fields for a specific provider
    pub fn fields_for_provider(provider: ProviderType) -> Vec<SettingsField> {
        let mut fields = vec![SettingsField::Provider];

        match provider {
            ProviderType::Azure => {
                fields.extend([
                    SettingsField::AzureUrl,
                    SettingsField::AzureApiKey,
                    SettingsField::AzureDeployment,
                    SettingsField::AzureApiVersion,
                ]);
            }
            ProviderType::OpenAI => {
                fields.extend([
                    SettingsField::OpenAIApiKey,
                    SettingsField::OpenAIModel,
                ]);
            }
            ProviderType::Anthropic => {
                fields.extend([
                    SettingsField::AnthropicApiKey,
                    SettingsField::AnthropicModel,
                ]);
            }
        }

        fields.extend([SettingsField::Save, SettingsField::Cancel]);
        fields
    }

    pub fn label(&self) -> &'static str {
        match self {
            SettingsField::Provider => "Provider",
            SettingsField::AzureUrl => "Azure URL",
            SettingsField::AzureApiKey => "API Key",
            SettingsField::AzureDeployment => "Deployment",
            SettingsField::AzureApiVersion => "API Version",
            SettingsField::OpenAIApiKey => "API Key",
            SettingsField::OpenAIModel => "Model",
            SettingsField::AnthropicApiKey => "API Key",
            SettingsField::AnthropicModel => "Model",
            SettingsField::Save => "Save",
            SettingsField::Cancel => "Cancel",
        }
    }

    pub fn is_button(&self) -> bool {
        matches!(self, SettingsField::Save | SettingsField::Cancel)
    }

    pub fn is_provider_selector(&self) -> bool {
        matches!(self, SettingsField::Provider)
    }

    pub fn is_password(&self) -> bool {
        matches!(
            self,
            SettingsField::AzureApiKey | SettingsField::OpenAIApiKey | SettingsField::AnthropicApiKey
        )
    }
}
