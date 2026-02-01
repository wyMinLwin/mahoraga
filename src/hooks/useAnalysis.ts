import { useState, useCallback } from 'react';
import type { AnalysisResult, Config } from '../types.js';
import { createProvider } from '../providers/index.js';

interface UseAnalysisReturn {
  result: AnalysisResult | null;
  isAnalyzing: boolean;
  error: string | null;
  analyze: (prompt: string) => Promise<void>;
  clear: () => void;
}

export function useAnalysis(config: Config | null): UseAnalysisReturn {
  const [result, setResult] = useState<AnalysisResult | null>(null);
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const analyze = useCallback(async (prompt: string) => {
    if (!config) {
      setError('Configuration required');
      return;
    }

    setIsAnalyzing(true);
    setError(null);
    setResult(null);

    try {
      const provider = createProvider(config);
      const analysisResult = await provider.analyze(prompt);
      setResult(analysisResult);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Analysis failed');
    } finally {
      setIsAnalyzing(false);
    }
  }, [config]);

  const clear = useCallback(() => {
    setResult(null);
    setError(null);
  }, []);

  return {
    result,
    isAnalyzing,
    error,
    analyze,
    clear,
  };
}
