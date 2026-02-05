mod azure;
mod openai;
mod anthropic;

pub use azure::AzureProvider;
pub use openai::OpenAIProvider;
pub use anthropic::AnthropicProvider;

use anyhow::Result;
use async_trait::async_trait;

use crate::types::{AnalysisResult, Config, ProviderType};

/// System prompt for analyzing prompts
pub const SYSTEM_PROMPT: &str = r#"You are a prompt quality analyzer. Analyze the given prompt and provide:
1. A quality score from 0-100
2. A list of specific improvements
3. A list of unclear or ambiguous parts

Respond in JSON format only:
{
  "score": <number 0-100>,
  "improvements": ["improvement 1", "improvement 2", ...],
  "unclear_parts": ["unclear part 1", "unclear part 2", ...]
}

Scoring guidelines:
- 90-100: Excellent - clear, specific, well-structured
- 70-89: Good - mostly clear with minor improvements needed
- 50-69: Fair - needs clarification or more specificity
- 30-49: Poor - significant ambiguity or missing context
- 0-29: Very poor - vague or incomprehensible

Be constructive and specific in your feedback."#;

/// Provider trait for LLM implementations
#[async_trait]
pub trait Provider: Send + Sync {
    /// Analyze a prompt and return the analysis result
    async fn analyze(&self, prompt: &str) -> Result<AnalysisResult>;

    /// Get the provider type
    fn provider_type(&self) -> ProviderType;
}

/// Create a provider based on configuration
pub fn create_provider(config: &Config) -> Result<Box<dyn Provider>> {
    match config.provider.active {
        ProviderType::Azure => {
            Ok(Box::new(AzureProvider::new(config.azure.clone())))
        }
        ProviderType::OpenAI => {
            Ok(Box::new(OpenAIProvider::new(config.openai.clone())))
        }
        ProviderType::Anthropic => {
            Ok(Box::new(AnthropicProvider::new(config.anthropic.clone())))
        }
    }
}

/// Parse JSON response from LLM into AnalysisResult
pub fn parse_analysis_response(response: &str) -> Result<AnalysisResult> {
    // Try to find JSON in the response (in case there's extra text)
    let json_str = if let Some(start) = response.find('{') {
        if let Some(end) = response.rfind('}') {
            &response[start..=end]
        } else {
            response
        }
    } else {
        response
    };

    let result: AnalysisResult = serde_json::from_str(json_str)
        .map_err(|e| anyhow::anyhow!("Failed to parse LLM response as JSON: {}", e))?;

    Ok(result)
}
