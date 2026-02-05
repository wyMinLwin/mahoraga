use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use crate::types::{AnalysisResult, AnthropicConfig, ProviderType};
use super::{parse_analysis_response, Provider, SYSTEM_PROMPT};

const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_API_VERSION: &str = "2023-06-01";

pub struct AnthropicProvider {
    config: AnthropicConfig,
    client: Client,
}

impl AnthropicProvider {
    pub fn new(config: AnthropicConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl Provider for AnthropicProvider {
    async fn analyze(&self, prompt: &str) -> Result<AnalysisResult> {
        if self.config.api_key.is_empty() {
            anyhow::bail!("Anthropic API key is not configured");
        }

        let body = json!({
            "model": self.config.model,
            "max_tokens": 1000,
            "system": SYSTEM_PROMPT,
            "messages": [
                {
                    "role": "user",
                    "content": format!("Analyze this prompt:\n\n{}", prompt)
                }
            ]
        });

        let response = self
            .client
            .post(ANTHROPIC_API_URL)
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", ANTHROPIC_API_VERSION)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Anthropic")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Anthropic API error ({}): {}", status, error_text);
        }

        let response_json: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse Anthropic response")?;

        // Anthropic's response format is different from OpenAI
        let content = response_json["content"][0]["text"]
            .as_str()
            .context("No content in Anthropic response")?;

        parse_analysis_response(content)
    }

    fn provider_type(&self) -> ProviderType {
        ProviderType::Anthropic
    }
}
