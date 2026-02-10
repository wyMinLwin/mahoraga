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
pub const SYSTEM_PROMPT: &str = r#"You are a strict prompt quality analyzer. Your job is to critically evaluate prompts with high standards. Analyze the given prompt and provide:
1. A quality score from 0-100 (be strict - most prompts should score below 70)
2. A list of specific, actionable improvements
3. A list of unclear or ambiguous parts

Respond in JSON format only:
{
  "score": <number 0-100>,
  "improvements": ["improvement 1", "improvement 2", ...],
  "unclear_parts": ["unclear part 1", "unclear part 2", ...]
}

STRICT Scoring Criteria (apply rigorously):

90-100 (Exceptional - RARE):
- Crystal clear objective with measurable success criteria
- Complete context: who, what, why, constraints, format
- Specific examples or templates provided
- Explicitly handles edge cases
- Professional-grade prompt engineering

70-89 (Good):
- Clear main objective
- Sufficient context for the task
- Specifies output format
- Minor ambiguities only

50-69 (Acceptable):
- Understandable intent but lacks specificity
- Missing important context or constraints
- Vague about expected output format
- Requires assumptions to complete

30-49 (Poor):
- Ambiguous or multiple possible interpretations
- Missing critical information
- No clear success criteria
- Would likely produce inconsistent results

0-29 (Inadequate):
- Extremely vague or single-word prompts
- Incomprehensible or contradictory
- No actionable information
- Impossible to produce useful output

IMPORTANT: Be critical. A simple one-liner like "Write code for X" should score 30-50 at most. Prompts need context, constraints, and clarity to score above 70. Reserve 90+ for truly exceptional prompts only.

Always provide at least 2-3 specific improvements, even for good prompts."#;

/// Provider trait for LLM implementations
#[async_trait]
pub trait Provider: Send + Sync {
    /// Analyze a prompt and return the analysis result
    async fn analyze(&self, prompt: &str) -> Result<AnalysisResult>;

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
