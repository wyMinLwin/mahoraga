import { useState, useCallback, useEffect } from 'react';
import type { Config } from '../types.js';
import {
  readConfig,
  writeConfig,
  resetConfig as resetConfigFile,
  isConfigured as checkConfigured,
  getDefaultConfig,
} from '../utils/config.js';

interface UseConfigReturn {
  config: Config | null;
  isConfigured: boolean;
  isLoading: boolean;
  saveConfig: (config: Config) => void;
  resetConfig: () => void;
  reloadConfig: () => void;
}

export function useConfig(): UseConfigReturn {
  const [config, setConfig] = useState<Config | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  const loadConfig = useCallback(() => {
    const loaded = readConfig();
    setConfig(loaded ?? getDefaultConfig());
    setIsLoading(false);
  }, []);

  useEffect(() => {
    loadConfig();
  }, [loadConfig]);

  const saveConfig = useCallback((newConfig: Config) => {
    writeConfig(newConfig);
    setConfig(newConfig);
  }, []);

  const resetConfig = useCallback(() => {
    resetConfigFile();
    const defaultConfig = getDefaultConfig();
    setConfig(defaultConfig);
  }, []);

  const reloadConfig = useCallback(() => {
    loadConfig();
  }, [loadConfig]);

  return {
    config,
    isConfigured: checkConfigured(config),
    isLoading,
    saveConfig,
    resetConfig,
    reloadConfig,
  };
}
