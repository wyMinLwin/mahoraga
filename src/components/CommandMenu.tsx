import React from 'react';
import { Box, Text } from 'ink';
import { COLORS } from '../utils/theme.js';

export interface Command {
  name: string;
  description: string;
}

export const COMMANDS: Command[] = [
  { name: '/settings', description: 'Configure API settings' },
  { name: '/default', description: 'Reset settings to defaults' },
  { name: '/clear', description: 'Clear current analysis' },
  { name: '/exit', description: 'Exit the application' },
];

interface CommandMenuProps {
  selectedIndex: number;
  filter: string;
}

export function CommandMenu({ selectedIndex, filter }: CommandMenuProps) {
  const filteredCommands = COMMANDS.filter((cmd) =>
    cmd.name.toLowerCase().startsWith(filter.toLowerCase())
  );

  if (filteredCommands.length === 0) {
    return null;
  }

  return (
    <Box flexDirection="column" marginTop={1}>
      {filteredCommands.map((cmd, index) => (
        <Box key={cmd.name}>
          <Box width={20}>
            <Text color={index === selectedIndex ? COLORS.primary : COLORS.secondary}>
              {cmd.name}
            </Text>
          </Box>
          <Text color={COLORS.muted}>{cmd.description}</Text>
        </Box>
      ))}
    </Box>
  );
}

export function getFilteredCommands(filter: string): Command[] {
  return COMMANDS.filter((cmd) =>
    cmd.name.toLowerCase().startsWith(filter.toLowerCase())
  );
}
