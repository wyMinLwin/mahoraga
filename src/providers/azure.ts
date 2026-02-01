import { BaseProvider } from './base.js';
import type { AnalysisResult } from '../types.js';

const SYSTEM_PROMPT = `You are a prompt analysis expert. Your task is to evaluate prompts that will be given to AI agents and provide structured feedback.

Analyze the user's prompt and respond with a JSON object containing:
1. "score": A number between 0 and 1 indicating how well an AI agent would understand and execute the prompt (1 = perfect clarity)
2. "improvements": An array of specific suggestions to make the prompt clearer or more effective
3. "unclearParts": An array of phrases or sections that are ambiguous or vague

Consider these factors when scoring:
- Clarity of instructions
- Specificity of requirements
- Defined output format
- Edge case handling
- Context provided

Respond ONLY with valid JSON, no markdown or additional text.

Example response:
{
  "score": 0.72,
  "improvements": ["Specify the expected output format", "Define what 'handle errors appropriately' means"],
  "unclearParts": ["'as needed' is vague", "'good performance' lacks metrics"]
}`;

export class AzureProvider extends BaseProvider {
  async analyze(prompt: string): Promise<AnalysisResult> {
    const { url, apiKey, deployment, apiVersion } = this.config;

    const endpoint = `${url.replace(/\/$/, '')}/openai/deployments/${deployment}/chat/completions?api-version=${apiVersion}`;

    const response = await fetch(endpoint, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'api-key': apiKey,
      },
      body: JSON.stringify({
        messages: [
          { role: 'system', content: SYSTEM_PROMPT },
          { role: 'user', content: prompt },
        ],
        temperature: 0.3,
        max_tokens: 1000,
      }),
    });

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(`Azure API error: ${response.status} - ${errorText}`);
    }

    const data = await response.json() as {
      choices: Array<{
        message: {
          content: string;
        };
      }>;
    };

    const content = data.choices[0]?.message?.content;
    if (!content) {
      throw new Error('No response from Azure API');
    }

    try {
      const result = JSON.parse(content) as AnalysisResult;

      // Validate the response structure
      if (typeof result.score !== 'number' || result.score < 0 || result.score > 1) {
        throw new Error('Invalid score in response');
      }
      if (!Array.isArray(result.improvements)) {
        result.improvements = [];
      }
      if (!Array.isArray(result.unclearParts)) {
        result.unclearParts = [];
      }

      return result;
    } catch {
      throw new Error('Failed to parse API response as JSON');
    }
  }
}
