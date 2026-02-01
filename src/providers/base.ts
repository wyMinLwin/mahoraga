import type { AnalysisResult, Config } from '../types.js';

export abstract class BaseProvider {
  protected config: Config;

  constructor(config: Config) {
    this.config = config;
  }

  abstract analyze(prompt: string): Promise<AnalysisResult>;
}
