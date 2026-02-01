import React from 'react';
import { Box, Text } from 'ink';
import { COLORS } from '../utils/theme.js';

const LOGO = [
  '◯─◯─◯',
  '◯ ◎ ◯',
  '◯─◯─◯',
];

export function Header() {
  return (
    <Box>
      <Box flexDirection="column" marginRight={2}>
        {LOGO.map((line, i) => (
          <Text key={i} color={COLORS.primary}>{line}</Text>
        ))}
      </Box>
      <Box flexDirection="column" justifyContent="center">
        <Box flexDirection="column" justifyContent="flex-end">
          <Text bold color={COLORS.primary}>MAHORAGA</Text>
          <Text color={COLORS.muted}>v1.0.0</Text>
        </Box>
      </Box>
    </Box>
  );
}
