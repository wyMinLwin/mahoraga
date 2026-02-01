import React from 'react';
import { Box, Text } from 'ink';
import { COLORS } from '../utils/theme.js';

interface CommandBarProps {
  screen: 'main' | 'settings';
}

export function CommandBar({ screen }: CommandBarProps) {
  if (screen === 'settings') {
    return (
      <Box marginTop={1} paddingX={1}>
        <Text color={COLORS.primary}>Tab next  Enter save  Esc cancel</Text>
      </Box>
    );
  }

  return (
    <Box marginTop={1} paddingX={1}>
      <Text color={COLORS.primary}>/settings  /default  /clear  Ctrl+C</Text>
    </Box>
  );
}
