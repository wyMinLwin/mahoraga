use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use crate::types::{AnalysisResult, OpenAIConfig, ProviderType};
use super::{parse_analysis_response, Provider, SYSTEM_PROMPT};

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";

pub struct OpenAIProvider {
    config: OpenAIConfig,
    client: Client,
}

impl OpenAIProvider {
    pub fn new(config: OpenAIConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    async fn analyze(&self, prompt: &str) -> Result<AnalysisResult> {
        if self.config.api_key.is_empty() {
            anyhow::bail!("OpenAI API key is not configured");
        }

        let body = json!({
            "model": self.config.model,
            "messages": [
                {
                    "role": "system",
                    "content": SYSTEM_PROMPT
                },
                {
                    "role": "user",
                    "content": format!("Analyze this prompt:\n\n{}", prompt)
                }
            ],
            "temperature": 0.3,
            "max_tokens": 1000
        });

        let response = self
            .client
            .post(OPENAI_API_URL)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to OpenAI")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("OpenAI API error ({}): {}", status, error_text);
        }

        let response_json: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse OpenAI response")?;

        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .context("No content in OpenAI response")?;

        parse_analysis_response(content)
    }

    fn provider_type(&self) -> ProviderType {
        ProviderType::OpenAI
    }
}
