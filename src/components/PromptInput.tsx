import React from 'react';
import { Box, Text } from 'ink';
import TextInput from 'ink-text-input';
import { COLORS } from '../utils/theme.js';

interface PromptInputProps {
  value: string;
  onChange: (value: string) => void;
  onSubmit: (value: string) => void;
  disabled?: boolean;
}

export function PromptInput({ value, onChange, onSubmit, disabled }: PromptInputProps) {
  return (
    <Box flexDirection="column" borderStyle="round" borderColor={COLORS.muted} paddingX={1}>
      <Box>
        <Text color={COLORS.primary}>{'>'}</Text>
        <Text> </Text>
        <TextInput
          value={value}
          onChange={onChange}
          onSubmit={onSubmit}
          focus={!disabled}
          placeholder=""
        />
      </Box>
    </Box>
  );
}
