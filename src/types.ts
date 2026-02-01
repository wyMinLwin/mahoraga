export interface Config {
  url: string;
  apiKey: string;
  deployment: string;
  apiVersion: string;
}

export interface AnalysisResult {
  score: number;
  improvements: string[];
  unclearParts: string[];
}

export interface Provider {
  analyze(prompt: string): Promise<AnalysisResult>;
}

export type Screen = 'main' | 'settings';

export interface AppState {
  screen: Screen;
  config: Config | null;
  isConfigured: boolean;
}
