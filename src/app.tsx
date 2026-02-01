import React, { useState, useCallback, useEffect } from 'react';
import { Box, Text, useApp, useInput } from 'ink';
import Spinner from 'ink-spinner';
import { Header } from './components/Header.js';
import { PromptInput } from './components/PromptInput.js';
import { ScoreDisplay } from './components/ScoreDisplay.js';
import { Feedback } from './components/Feedback.js';
import { ConfigRequired } from './components/ConfigRequired.js';
import { Settings } from './components/Settings.js';
import { CommandMenu, getFilteredCommands } from './components/CommandMenu.js';
import { useConfig } from './hooks/useConfig.js';
import { useAnalysis } from './hooks/useAnalysis.js';
import { COLORS } from './utils/theme.js';
import type { Screen, Config } from './types.js';
import { getDefaultConfig } from './utils/config.js';

export function App() {
  const { exit } = useApp();
  const [screen, setScreen] = useState<Screen>('main');
  const [inputValue, setInputValue] = useState('');
  const [showConfigRequired, setShowConfigRequired] = useState(false);
  const [isExiting, setIsExiting] = useState(false);
  const [commandIndex, setCommandIndex] = useState(0);
  const [inputKey, setInputKey] = useState(0);
  const { config, isConfigured, isLoading, saveConfig, resetConfig } = useConfig();
  const { result, isAnalyzing, error, analyze, clear } = useAnalysis(config);

  const showCommandMenu = inputValue.startsWith('/') && screen === 'main' && !isAnalyzing;
  const filteredCommands = showCommandMenu ? getFilteredCommands(inputValue) : [];

  useEffect(() => {
    if (isExiting) {
      const timer = setTimeout(() => {
        exit();
      }, 500);
      return () => clearTimeout(timer);
    }
  }, [isExiting, exit]);

  useEffect(() => {
    setCommandIndex(0);
  }, [inputValue]);

  useInput((input, key) => {
    if (!showCommandMenu || filteredCommands.length === 0) return;

    if (key.downArrow) {
      setCommandIndex((prev) => (prev + 1) % filteredCommands.length);
    } else if (key.upArrow) {
      setCommandIndex((prev) => (prev - 1 + filteredCommands.length) % filteredCommands.length);
    } else if (key.tab) {
      const selectedCommand = filteredCommands[commandIndex];
      if (selectedCommand) {
        setInputValue(selectedCommand.name);
        setInputKey((k) => k + 1);
      }
    }
  }, { isActive: showCommandMenu });

  const handleInputChange = useCallback((value: string) => {
    setInputValue(value);
    setShowConfigRequired(false);
  }, []);

  const handleSubmit = useCallback(async (value: string) => {
    const trimmed = value.trim();

    // Handle commands
    if (trimmed === '/settings') {
      setInputValue('');
      setShowConfigRequired(false);
      setScreen('settings');
      return;
    }

    if (trimmed === '/default') {
      setInputValue('');
      resetConfig();
      clear();
      return;
    }

    if (trimmed === '/clear') {
      setInputValue('');
      clear();
      return;
    }

    if (trimmed === '/exit') {
      setIsExiting(true);
      return;
    }

    // Analyze prompt if configured
    if (trimmed && !trimmed.startsWith('/')) {
      if (isConfigured) {
        setInputValue('');
        await analyze(trimmed);
      } else {
        setShowConfigRequired(true);
      }
    }
  }, [isConfigured, resetConfig, clear, analyze]);

  const handleSettingsSave = useCallback((newConfig: Config) => {
    saveConfig(newConfig);
    setScreen('main');
  }, [saveConfig]);

  const handleSettingsCancel = useCallback(() => {
    setScreen('main');
  }, []);

  if (isExiting) {
    return (
      <Box padding={1}>
        <Text color={COLORS.muted}>disintegrated</Text>
      </Box>
    );
  }

  if (isLoading) {
    return (
      <Box flexDirection="column" padding={1}>
        <Header />
        <Box>
          <Text color={COLORS.primary}>
            <Spinner type="dots" />
          </Text>
          <Text> Loading...</Text>
        </Box>
      </Box>
    );
  }

  if (screen === 'settings') {
    return (
      <Box flexDirection="column" padding={1}>
        <Header />
        <Settings
          config={config ?? getDefaultConfig()}
          onSave={handleSettingsSave}
          onCancel={handleSettingsCancel}
        />
      </Box>
    );
  }

  return (
    <Box flexDirection="column" padding={1}>
      <Header />

      {showConfigRequired && <ConfigRequired />}

      <PromptInput
        key={inputKey}
        value={inputValue}
        onChange={handleInputChange}
        onSubmit={handleSubmit}
        disabled={isAnalyzing}
      />

      {showCommandMenu && filteredCommands.length > 0 && (
        <CommandMenu selectedIndex={commandIndex} filter={inputValue} />
      )}

      {isAnalyzing && (
        <Box marginY={1}>
          <Text color={COLORS.primary}>
            <Spinner type="dots" />
          </Text>
          <Text> Analyzing prompt...</Text>
        </Box>
      )}

      {error && (
        <Box marginY={1}>
          <Text color={COLORS.error}>Error: {error}</Text>
        </Box>
      )}

      {result && (
        <Box flexDirection="column" borderStyle="single" borderColor={COLORS.muted} paddingX={1}>
          <ScoreDisplay score={result.score} />
          <Feedback result={result} />
        </Box>
      )}
    </Box>
  );
}
