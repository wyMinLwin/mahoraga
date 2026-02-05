use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use crate::types::{AnalysisResult, AzureConfig, ProviderType};
use super::{parse_analysis_response, Provider, SYSTEM_PROMPT};

pub struct AzureProvider {
    config: AzureConfig,
    client: Client,
}

impl AzureProvider {
    pub fn new(config: AzureConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    fn build_url(&self) -> String {
        let base_url = self.config.url.trim_end_matches('/');
        format!(
            "{}/openai/deployments/{}/chat/completions?api-version={}",
            base_url, self.config.deployment, self.config.api_version
        )
    }
}

#[async_trait]
impl Provider for AzureProvider {
    async fn analyze(&self, prompt: &str) -> Result<AnalysisResult> {
        if self.config.url.is_empty() {
            anyhow::bail!("Azure URL is not configured");
        }
        if self.config.api_key.is_empty() {
            anyhow::bail!("Azure API key is not configured");
        }
        if self.config.deployment.is_empty() {
            anyhow::bail!("Azure deployment is not configured");
        }

        let url = self.build_url();

        let body = json!({
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
            .post(&url)
            .header("api-key", &self.config.api_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Azure OpenAI")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Azure API error ({}): {}", status, error_text);
        }

        let response_json: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse Azure response")?;

        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .context("No content in Azure response")?;

        parse_analysis_response(content)
    }

    fn provider_type(&self) -> ProviderType {
        ProviderType::Azure
    }
}
